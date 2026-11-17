use std::collections::BTreeSet;
use std::sync::Arc;

use ffs_content::{Effect, Incident, OptionKind, Pack};
use rand_core::RngCore;
use thiserror::Error;

use crate::consts::*;
use crate::rng;
use crate::state::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CommandError {
    #[error("the run is over")]
    RunEnded,
    #[error("no such ticket")]
    NoSuchTicket,
    #[error("no such option")]
    NoSuchOption,
    #[error("you have no dirt on that person")]
    NoDirt,
    #[error("no such scheme")]
    NoSuchScheme,
    #[error("you already ran that scheme today")]
    SchemeUsedToday,
    #[error("that scheme is not available yet")]
    SchemeLocked,
    #[error("no such story")]
    NoSuchStory,
}

pub struct Engine {
    pub(crate) pack: Arc<Pack>,
    pub(crate) s: State,
    log: Vec<Command>,
    ev: Vec<Event>,
}

impl Engine {
    /// Start a new run.
    pub fn new(pack: Arc<Pack>, seed: u64) -> Engine {
        let rivals = pack
            .rivals
            .iter()
            .map(|r| RivalState {
                id: r.id.clone(),
                np: 0,
            })
            .collect();
        let s = State {
            seed,
            rng_n: 0,
            step: 0,
            day: 0,
            minute: 0,
            fucks: START_FUCKS,
            susp: 0,
            cash: START_CASH,
            np: 0,
            queue: vec![],
            next_ticket: 1,
            used: BTreeSet::new(),
            chain_count: Default::default(),
            dirt: vec![],
            dirt_given: BTreeSet::new(),
            stories: vec![],
            next_story: 1,
            rivals,
            chat: vec![],
            np_log: vec![],
            fucks_history: vec![START_FUCKS],
            schemes_today: BTreeSet::new(),
            rants_today: 0,
            coffees_today: 0,
            covers_today: 0,
            boasts_today: 0,
            resolved: 0,
            toast: String::new(),
            last_cause: String::new(),
            ended: None,
        };
        let mut e = Engine {
            pack,
            s,
            log: vec![],
            ev: vec![],
        };
        for i in 0..e.pack.rivals.len() {
            let r = &e.pack.rivals[i];
            let (lo, hi) = (r.np_start[0] as i64, r.np_start[1] as i64);
            let v = e.range("rival_np_start", lo, hi) as i32;
            e.s.rivals[i].np = v;
        }
        e.sys_chat("-!- you joined #war-room. 4 other sysadmins are here, drinking your coffee.");
        e.start_day();
        e.ev.clear();
        e
    }

    /// Rebuild a run from its command log (replay/save/load).
    pub fn replay(pack: Arc<Pack>, seed: u64, log: &[Command]) -> Result<Engine, CommandError> {
        let mut e = Engine::new(pack, seed);
        for c in log {
            e.apply(c.clone())?;
        }
        Ok(e)
    }

    pub fn log(&self) -> &[Command] {
        &self.log
    }

    pub fn pack(&self) -> &Pack {
        &self.pack
    }

    pub fn state(&self) -> &State {
        &self.s
    }

    pub fn is_over(&self) -> bool {
        self.s.ended.is_some()
    }

    /// Hash of the whole game state (replays must reproduce it).
    pub fn state_hash(&self) -> String {
        let bytes = serde_json::to_vec(&self.s).unwrap_or_default();
        blake3::hash(&bytes).to_hex().to_string()
    }

    // ----- RNG helpers -----

    fn stream(&mut self, label: &str) -> rand_chacha::ChaCha8Rng {
        self.s.rng_n += 1;
        rng::stream(self.s.seed, self.s.rng_n, label)
    }

    pub(crate) fn range(&mut self, label: &str, lo: i64, hi: i64) -> i64 {
        let mut r = self.stream(label);
        rng::range(&mut r, lo, hi)
    }

    fn pct(&mut self, label: &str, p: u32) -> bool {
        self.range(label, 1, 100) <= p as i64
    }

    fn pick(&mut self, label: &str, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        let mut r = self.stream(label);
        (r.next_u64() % len as u64) as usize
    }

    // ----- Small helpers -----

    fn toast(&mut self, text: impl Into<String>) {
        let t = text.into();
        self.s.toast = t.clone();
        self.ev.push(Event::Toast { text: t });
    }

    fn sys_chat(&mut self, text: &str) {
        self.push_chat("***", text, "system");
    }

    fn push_chat(&mut self, who: &str, text: &str, kind: &str) {
        self.s.chat.push(ChatLine {
            who: who.to_string(),
            text: text.to_string(),
            kind: kind.to_string(),
        });
        let n = self.s.chat.len();
        if n > 60 {
            self.s.chat.drain(0..n - 60);
        }
    }

    fn add_np(&mut self, delta: i32, reason: &str) {
        if delta == 0 {
            return;
        }
        self.s.np = (self.s.np + delta).max(0);
        self.s.np_log.push(NpEntry {
            day: self.s.day,
            delta,
            reason: reason.to_string(),
        });
        let n = self.s.np_log.len();
        if n > 100 {
            self.s.np_log.drain(0..n - 100);
        }
    }

    fn add_fucks(&mut self, d: i32) {
        self.s.fucks = (self.s.fucks + d).clamp(0, MAX_FUCKS);
    }

    fn add_susp(&mut self, d: i32) {
        self.s.susp = (self.s.susp + d).clamp(0, 100);
    }

    // ----- Commands -----

    pub fn apply(&mut self, cmd: Command) -> Result<Vec<Event>, CommandError> {
        if self.s.ended.is_some() {
            return Err(CommandError::RunEnded);
        }
        self.validate(&cmd)?; // never mutates
        self.ev.clear();
        self.s.step += 1;
        self.log.push(cmd.clone());
        match cmd {
            Command::Resolve { ticket, option } => self.do_resolve(ticket, option as usize),
            Command::UseDirt { ticket } => self.do_use_dirt(ticket),
            Command::Rant { send } => self.do_rant(send),
            Command::Coffee => self.do_coffee(),
            Command::CoverTracks => self.do_cover(),
            Command::Scheme { id } => self.do_scheme(&id),
            Command::Boast { story } => self.do_boast(story),
            Command::HelpOpened => self.do_help(),
            Command::EndDay => {
                self.s.minute = DAY_MINUTES;
                self.after_action(0);
            }
            Command::Retire => self.do_retire(),
        }
        Ok(std::mem::take(&mut self.ev))
    }

    fn validate(&self, cmd: &Command) -> Result<(), CommandError> {
        match cmd {
            Command::Resolve { ticket, option } => {
                let t = self
                    .s
                    .queue
                    .iter()
                    .find(|t| t.id == *ticket)
                    .ok_or(CommandError::NoSuchTicket)?;
                let inc = self
                    .pack
                    .incident(&t.incident)
                    .ok_or(CommandError::NoSuchTicket)?;
                if (*option as usize) >= inc.options.len() {
                    return Err(CommandError::NoSuchOption);
                }
            }
            Command::UseDirt { ticket } => {
                let t = self
                    .s
                    .queue
                    .iter()
                    .find(|t| t.id == *ticket)
                    .ok_or(CommandError::NoSuchTicket)?;
                let inc = self
                    .pack
                    .incident(&t.incident)
                    .ok_or(CommandError::NoSuchTicket)?;
                if !self.s.dirt.iter().any(|d| d.who == inc.who) {
                    return Err(CommandError::NoDirt);
                }
            }
            Command::Scheme { id } => {
                if !self.pack.schemes.iter().any(|s| s.id == *id) {
                    return Err(CommandError::NoSuchScheme);
                }
                if self.s.schemes_today.contains(id) {
                    return Err(CommandError::SchemeUsedToday);
                }
                if !self.scheme_unlocked(id) {
                    return Err(CommandError::SchemeLocked);
                }
            }
            Command::Boast { story } if !self.s.stories.iter().any(|s| s.id == *story) => {
                return Err(CommandError::NoSuchStory);
            }
            _ => {}
        }
        Ok(())
    }

    fn do_resolve(&mut self, ticket: u32, option: usize) {
        let pack = self.pack.clone();
        let Some(pos) = self.s.queue.iter().position(|t| t.id == ticket) else {
            return;
        };
        let t = self.s.queue.remove(pos);
        let Some(inc) = pack.incident(&t.incident) else {
            return;
        };
        let Some(opt) = inc.options.get(option) else {
            return;
        };
        self.s.resolved += 1;
        self.s.last_cause = if inc.cause.is_empty() {
            "something small and stupid finally did it.".to_string()
        } else {
            inc.cause.clone()
        };
        for e in &opt.effects {
            self.apply_effect(e, &inc.who, &opt.text);
        }
        if let (Some(text), Some(v)) = (&opt.story, opt.story_value) {
            let risky = matches!(
                opt.kind,
                OptionKind::Malicious | OptionKind::Profitable | OptionKind::Leverage
            );
            self.add_story(text.clone(), v, risky);
        }
        let result = if opt.result.is_empty() {
            "Done.".to_string()
        } else {
            opt.result.clone()
        };
        self.toast(result);
        self.after_action(opt.minutes.unwrap_or(INCIDENT_MINUTES));
    }

    fn add_story(&mut self, text: String, value: u8, risky: bool) {
        let id = self.s.next_story;
        self.s.next_story += 1;
        self.s.stories.push(Story {
            id,
            text,
            value: value.clamp(1, 3),
            risky,
            day: self.s.day,
        });
        let n = self.s.stories.len();
        if n > 8 {
            self.s.stories.drain(0..n - 8);
        }
    }

    fn apply_effect(&mut self, e: &Effect, who: &str, why: &str) {
        if let Some(d) = e.fucks {
            self.add_fucks(d);
        }
        if let Some(d) = e.susp {
            self.add_susp(d);
        }
        if let Some(d) = e.cash {
            self.s.cash = (self.s.cash as i64 + d).clamp(0, i32::MAX as i64) as i32;
        }
        if let Some(d) = e.np {
            self.add_np(d, why);
        }
        if e.dirt == Some(true) {
            self.give_dirt(who);
        }
        if let Some(id) = &e.spawn {
            self.spawn_incident(id);
        }
        if let (Some(c), Some(then)) = (e.chance, &e.then) {
            if self.pct("chance", c) {
                for x in then {
                    self.apply_effect(x, who, why);
                }
            }
        }
    }

    fn give_dirt(&mut self, who: &str) {
        let pack = self.pack.clone();
        let free: Vec<usize> = pack
            .dirt
            .iter()
            .enumerate()
            .filter(|(i, d)| d.who == who && !self.s.dirt_given.contains(i))
            .map(|(i, _)| i)
            .collect();
        if free.is_empty() {
            return;
        }
        let i = free[self.pick("dirt", free.len())];
        let d = &pack.dirt[i];
        self.s.dirt_given.insert(i);
        self.s.dirt.push(Holding {
            who: d.who.clone(),
            label: d.label.clone(),
            use_line: d.use_line.clone(),
            idx: i,
        });
        let name = d.name.clone().unwrap_or_else(|| d.who.clone());
        self.toast(format!("DIRT on {name}: {}", d.label));
    }

    fn do_use_dirt(&mut self, ticket: u32) {
        let pack = self.pack.clone();
        let Some(pos) = self.s.queue.iter().position(|t| t.id == ticket) else {
            return;
        };
        let t = self.s.queue.remove(pos);
        let Some(inc) = pack.incident(&t.incident) else {
            return;
        };
        let Some(hp) = self.s.dirt.iter().position(|d| d.who == inc.who) else {
            return;
        };
        let h = self.s.dirt.remove(hp);
        self.s.resolved += 1;
        self.s.last_cause = if inc.cause.is_empty() {
            "a pile of unresolved tickets.".to_string()
        } else {
            inc.cause.clone()
        };
        self.add_fucks(10);
        self.add_np(2, "played dirt");
        let name = pack
            .employee(&inc.who)
            .map(|e| e.name.clone())
            .unwrap_or_default();
        self.add_story(
            format!("made {name} withdraw a ticket with one raised eyebrow"),
            2,
            true,
        );
        self.toast(format!("{} {name} withdraws the ticket.", h.use_line));
        self.after_action(DIRT_MINUTES);
    }

    fn do_rant(&mut self, send: bool) {
        let n = self.s.rants_today;
        self.s.rants_today += 1;
        let base = if send { 10 } else { 5 };
        let gain = match n {
            0 | 1 => base,
            2 | 3 => base / 2,
            _ => 1,
        };
        self.add_fucks(gain);
        if send {
            self.add_susp(5);
            self.toast(format!(
                "You hit SEND. +{gain} fucks. It felt great. It was a mistake."
            ));
            if self.pct("rant_hr", 30) {
                self.spawn_incident("sys_hr_meeting");
            }
        } else {
            self.toast(format!(
                "You delete the draft. +{gain} fucks. Nobody ever knows."
            ));
        }
        self.after_action(RANT_MINUTES);
    }

    fn do_coffee(&mut self) {
        let gain = match self.s.coffees_today {
            0 => 4,
            1 => 2,
            _ => 0,
        };
        self.s.coffees_today += 1;
        self.add_fucks(gain);
        self.toast(format!("Coffee. It is brown and hot. +{gain} fucks."));
        self.after_action(COFFEE_MINUTES);
    }

    /// Whether a scheme's `min_day` has been reached.
    pub fn scheme_unlocked(&self, id: &str) -> bool {
        self.pack
            .schemes
            .iter()
            .find(|s| s.id == id)
            .is_some_and(|s| self.s.day >= s.min_day)
    }

    /// Suspicion the next "cover your tracks" today would remove.
    pub fn cover_gain(&self) -> i32 {
        COVER_SUSP[(self.s.covers_today as usize).min(COVER_SUSP.len() - 1)]
    }

    fn do_cover(&mut self) {
        let pack = self.pack.clone();
        let want = self.cover_gain();
        let before = self.s.susp;
        self.s.covers_today += 1;
        self.add_susp(-want);
        self.add_fucks(-COVER_FUCKS);
        let got = before - self.s.susp;
        let lines = &pack.quips.cover;
        let line = lines
            .get(self.pick("cover_line", lines.len()))
            .cloned()
            .unwrap_or_else(|| "You tidy the logs.".to_string());
        self.toast(format!(
            "{line} Suspicion -{got}. -{COVER_FUCKS} fucks (it was tedious)."
        ));
        self.after_action(COVER_MINUTES);
    }

    fn do_scheme(&mut self, id: &str) {
        let pack = self.pack.clone();
        let Some(sc) = pack.schemes.iter().find(|s| s.id == id) else {
            return;
        };
        self.s.schemes_today.insert(sc.id.clone());
        let base = self.range("scheme_cash", sc.cash[0], sc.cash[1]);
        let mut scaled = base * (100 + SCHEME_GROWTH_PCT * (self.s.day as i64 - 1)) / 100;
        let jackpot = sc.jackpot_chance > 0 && self.pct("jackpot", sc.jackpot_chance);
        let mut susp = sc.susp;
        if jackpot {
            let m = self.range("jackpot_mult", sc.jackpot_mult[0], sc.jackpot_mult[1]);
            scaled *= m;
            susp += sc.jackpot_susp;
        }
        self.s.cash = (self.s.cash as i64 + scaled).clamp(0, i32::MAX as i64) as i32;
        self.add_susp(susp);
        let value = if jackpot { 3 } else { sc.story_value };
        self.add_story(sc.story.clone(), value, true);
        if jackpot {
            self.toast(format!(
                "JACKPOT! {} +${scaled}. Suspicion +{susp}.",
                sc.jackpot_line
            ));
        } else {
            self.toast(format!("{}: +${scaled}. Suspicion +{susp}.", sc.name));
        }
        if self.pct("audit", sc.audit_chance) {
            self.spawn_incident("sys_audit_scare");
        }
        self.after_action(sc.minutes);
    }

    fn do_boast(&mut self, story: u32) {
        let pack = self.pack.clone();
        let Some(pos) = self.s.stories.iter().position(|s| s.id == story) else {
            return;
        };
        let st = self.s.stories.remove(pos);
        self.s.boasts_today += 1;
        let mut v = st.value as i32;
        if self.s.boasts_today > 2 {
            v = (v - 1).max(1);
        }
        self.push_chat("you", &format!("I {}", st.text), "you");
        let mut gain = 0;
        for i in 0..pack.rivals.len() {
            let r = &pack.rivals[i];
            let ego = r.ego as i32;
            let (pool, g) = if v > ego {
                (&r.react_high, v * 2)
            } else if v == ego {
                (&r.react_even, v)
            } else {
                (&r.react_low, 0)
            };
            let line = pool
                .get(self.pick("react", pool.len()))
                .cloned()
                .unwrap_or_default();
            self.push_chat(&r.nick, &line, "rival");
            if g == 0 {
                self.s.rivals[i].np += 1;
            }
            gain += g;
        }
        self.add_np(gain, &format!("boast: I {}", st.text));
        if st.risky {
            self.add_susp(6);
            self.toast(format!(
                "Boast landed: NP +{gain}. Suspicion +6 (you said the quiet part out loud)."
            ));
        } else {
            self.toast(format!("Boast landed: NP +{gain}."));
        }
        self.after_action(BOAST_MINUTES);
    }

    fn do_help(&mut self) {
        self.add_np(-1, "opened F1 help (RTFM)");
        let pack = self.pack.clone();
        if !pack.rivals.is_empty() {
            let i = self.pick("mock", pack.rivals.len());
            let r = &pack.rivals[i];
            let line = r
                .mock_help
                .get(self.pick("mock_line", r.mock_help.len()))
                .cloned()
                .unwrap_or_default();
            self.push_chat(&r.nick, &line, "rival");
        }
        self.toast("You asked for help. -1 NP.");
    }

    fn do_retire(&mut self) {
        let kind = if self.s.cash >= RETIRE_GOAL && self.s.susp < RETIRE_MAX_SUSP {
            EndKind::Retired
        } else {
            EndKind::Ran
        };
        self.end_run(kind, "you took the money and ran.");
    }

    // ----- Time, days, director -----

    fn after_action(&mut self, minutes: u32) {
        self.s.minute += minutes;
        if self.s.ended.is_none() && self.s.queue.len() < MAX_QUEUE && self.pct("arrival", 45) {
            self.spawn_random();
        }
        if self.pct("chatter", 30) {
            self.rival_chatter(false);
        }
        self.check_end();
        if self.s.ended.is_some() {
            return;
        }
        let f = self.s.fucks;
        self.s.fucks_history.push(f);
        if self.s.fucks_history.len() > 24 {
            self.s.fucks_history.remove(0);
        }
        if self.s.minute >= DAY_MINUTES {
            self.end_day();
        }
    }

    fn check_end(&mut self) {
        if self.s.ended.is_some() {
            return;
        }
        if self.s.fucks <= 0 {
            let cause = self.s.last_cause.clone();
            self.end_run(EndKind::Meltdown, &cause);
        } else if self.s.susp >= 100 {
            self.end_run(
                EndKind::Fired,
                "the auditors finally read your expense reports.",
            );
        }
    }

    fn end_day(&mut self) {
        let pack = self.pack.clone();
        let waiting = self.s.queue.len() as i32;
        for t in &mut self.s.queue {
            t.age += 1;
        }
        let penalty = (waiting * 2).min(8);
        self.add_fucks(-penalty);
        let q = &pack.quips.day_end;
        let quip = q
            .get(self.pick("quip", q.len()))
            .cloned()
            .unwrap_or_default();
        let day = self.s.day;
        self.ev.push(Event::DayEnded { day, quip });
        let cooled = self.s.susp.min(SUSP_DECAY_PER_DAY);
        if penalty > 0 {
            self.toast(format!(
                "{waiting} tickets left waiting overnight: -{penalty} fucks. Suspicion cooled -{cooled}."
            ));
        } else if cooled > 0 {
            self.toast(format!("Overnight, Suspicion cooled -{cooled}."));
        }
        if self.s.fucks <= 0 {
            self.s.last_cause = "death by a thousand unanswered tickets.".to_string();
            let cause = self.s.last_cause.clone();
            self.end_run(EndKind::Meltdown, &cause);
            return;
        }
        self.start_day();
        if self.s.fucks <= 0 {
            self.end_run(
                EndKind::Meltdown,
                "the sheer accumulated weight of another working day.",
            );
        }
    }

    fn start_day(&mut self) {
        let pack = self.pack.clone();
        self.s.day += 1;
        self.s.minute = 0;
        self.s.schemes_today.clear();
        self.s.rants_today = 0;
        self.s.coffees_today = 0;
        self.s.covers_today = 0;
        self.s.boasts_today = 0;
        if self.s.day > 1 {
            let d = self.s.day as i32;
            self.add_fucks(-(EROSION_PER_DAY + (d - 2).max(0) + d * d / 4));
            self.add_susp(-SUSP_DECAY_PER_DAY);
        }
        // rivals grow and post
        for i in 0..pack.rivals.len() {
            let r = &pack.rivals[i];
            let g = self.range(
                "rival_growth",
                r.np_per_day[0] as i64,
                r.np_per_day[1] as i64,
            ) as i32;
            self.s.rivals[i].np += g;
        }
        self.sys_chat(&format!("--- Day {} ---", self.s.day));
        self.rival_chatter(true);
        self.rival_chatter(true);
        // tickets
        let want = (3 + self.s.day as usize / 3).min(MAX_QUEUE);
        let mut tries = 0;
        while self.s.queue.len() < want && tries < 12 {
            self.spawn_random();
            tries += 1;
        }
        if self.s.queue.is_empty() {
            self.spawn_random();
        }
        if self.s.day >= 2
            && !self.s.used.contains("sys_coffee_machine")
            && self.pct("coffee_machine", 20)
        {
            self.s.used.insert("sys_coffee_machine".to_string());
            self.spawn_incident("sys_coffee_machine");
        }
        if self.s.susp >= 60 && self.pct("auditor", 40) {
            self.spawn_incident("sys_auditor_visit");
        }
        self.s.fucks_history.push(self.s.fucks);
        if self.s.fucks_history.len() > 24 {
            self.s.fucks_history.remove(0);
        }
    }

    fn rival_chatter(&mut self, boasty: bool) {
        let pack = self.pack.clone();
        if pack.rivals.is_empty() {
            return;
        }
        let i = self.pick("rival_who", pack.rivals.len());
        let r = &pack.rivals[i];
        if boasty && !r.boast.is_empty() && self.pct("rival_boast", 60) {
            let b = &r.boast[self.pick("rival_boast_line", r.boast.len())];
            self.s.rivals[i].np += b.np / 2;
            self.push_chat(&r.nick, &b.text.clone(), "rival");
        } else if !r.idle.is_empty() {
            let l = r.idle[self.pick("rival_idle", r.idle.len())].clone();
            self.push_chat(&r.nick, &l, "rival");
        }
    }

    fn current_tier(&self) -> u8 {
        (1 + (self.s.day.saturating_sub(1)) / 3).clamp(1, 3) as u8
    }

    fn eligible(&self, inc: &Incident, cur: u8) -> u32 {
        if inc.system {
            return 0;
        }
        if let Some(c) = &inc.chain {
            let seen = self.s.chain_count.get(c).copied().unwrap_or(0);
            if inc.step.unwrap_or(0) != seen {
                return 0;
            }
            if self.s.queue.iter().any(|t| {
                self.pack
                    .incident(&t.incident)
                    .and_then(|i| i.chain.as_ref())
                    == Some(c)
            }) {
                return 0;
            }
            if inc.tier > cur + 1 {
                return 0;
            }
            return if seen > 0 { 3 } else { 2 };
        }
        if self.s.used.contains(&inc.id) || self.s.queue.iter().any(|t| t.incident == inc.id) {
            return 0;
        }
        if inc.tier == cur {
            4
        } else if inc.tier < cur {
            2
        } else if inc.tier == cur + 1 {
            1
        } else {
            0
        }
    }

    fn spawn_random(&mut self) {
        let pack = self.pack.clone();
        let cur = self.current_tier();
        let mut cands: Vec<(usize, u32)> = pack
            .incidents
            .iter()
            .enumerate()
            .map(|(i, inc)| (i, self.eligible(inc, cur)))
            .filter(|(_, w)| *w > 0)
            .collect();
        if cands.is_empty() {
            // pool exhausted: forget standalone history and try again
            self.s.used.clear();
            cands = pack
                .incidents
                .iter()
                .enumerate()
                .map(|(i, inc)| (i, self.eligible(inc, cur)))
                .filter(|(_, w)| *w > 0)
                .collect();
        }
        let total: u32 = cands.iter().map(|(_, w)| *w).sum();
        if total == 0 {
            return;
        }
        let mut roll = self.range("director", 0, total as i64 - 1) as u32;
        for (i, w) in cands {
            if roll < w {
                self.push_ticket(&pack.incidents[i].id.clone());
                return;
            }
            roll -= w;
        }
    }

    fn spawn_incident(&mut self, id: &str) {
        if self.pack.incident(id).is_some() {
            self.push_ticket(id);
        }
    }

    fn push_ticket(&mut self, id: &str) {
        let Some(inc) = self.pack.incident(id) else {
            return;
        };
        let mut nth = 1;
        if let Some(c) = &inc.chain {
            let n = self.s.chain_count.entry(c.clone()).or_insert(0);
            *n += 1;
            nth = inc.step.unwrap_or(0) + 1;
        } else {
            self.s.used.insert(id.to_string());
        }
        if self.s.queue.len() >= MAX_QUEUE {
            // overflow: oldest ticket escalates to management
            let old = self.s.queue.remove(0);
            self.add_fucks(-3);
            self.toast(format!(
                "Ticket #{} escalated to management: -3 fucks.",
                old.id
            ));
        }
        let t = Ticket {
            id: self.s.next_ticket,
            incident: id.to_string(),
            age: 0,
            nth,
        };
        self.s.next_ticket += 1;
        self.s.queue.push(t);
    }

    fn end_run(&mut self, kind: EndKind, cause: &str) {
        let pack = self.pack.clone();
        let kind_s = match kind {
            EndKind::Meltdown => "meltdown",
            EndKind::Fired => "fired",
            EndKind::Retired => "retire",
            EndKind::Ran => "ran",
        };
        let ids: Vec<&ffs_content::Ending> =
            pack.endings.iter().filter(|e| e.kind == kind_s).collect();
        let ending = ids
            .get(self.pick("ending", ids.len()))
            .map(|e| e.id.clone())
            .unwrap_or_default();
        self.s.ended = Some(Ended {
            kind,
            ending,
            cause: cause.to_string(),
        });
        self.ev.push(Event::RunEnded { kind });
    }
}
