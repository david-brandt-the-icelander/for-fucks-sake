//! The view model: everything the UI needs, as JSON. Types are exported to TypeScript.

use ffs_content::{Effect, OptionKind};
use serde::Serialize;
use ts_rs::TS;

use crate::consts::*;
use crate::engine::Engine;
use crate::state::*;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct Chip {
    pub text: String,
    /// "good" | "bad" | "neutral"
    pub tone: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct OptionView {
    pub n: u8,
    pub text: String,
    pub hint: Vec<Chip>,
    pub kind: OptionKind,
    pub temptation: bool,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct DirtOffer {
    pub name: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct TicketView {
    pub id: u32,
    pub title: String,
    pub who_id: String,
    pub who_name: String,
    pub who_role: String,
    pub nth: u32,
    pub age: u32,
    pub quote: String,
    pub details: String,
    pub agitated: bool,
    pub portrait: String,
    pub options: Vec<OptionView>,
    pub dirt: Option<DirtOffer>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct SchemeView {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub minutes: u32,
    pub hint: Vec<Chip>,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct StoryView {
    pub id: u32,
    pub text: String,
    pub value: u8,
    pub risky: bool,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct RankRow {
    pub nick: String,
    pub corp: String,
    pub np: i32,
    pub you: bool,
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct DirtView {
    pub name: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct EndView {
    pub kind: EndKind,
    pub title: String,
    pub lines: Vec<String>,
    pub cause: String,
    pub days: u32,
    pub cash: i32,
    pub np: i32,
    pub rank: u32,
    pub nick_title: String,
    pub share: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct Header {
    pub corp: String,
    pub day: u32,
    pub clock: String,
    pub minutes_left: u32,
    pub cash: i32,
    pub retire_goal: i32,
    /// F10 right now would be a true retirement (otherwise it is "took the money and ran").
    pub can_retire: bool,
    pub susp: i32,
    pub np: i32,
    pub title: String,
    pub rank: u32,
    pub fucks: i32,
    pub fucks_history: Vec<i32>,
    pub mood: String,
    pub toast: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct Today {
    pub rants: u32,
    pub coffees: u32,
    pub boasts: u32,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct View {
    pub header: Header,
    pub tickets: Vec<TicketView>,
    pub schemes: Vec<SchemeView>,
    pub stories: Vec<StoryView>,
    pub board: Vec<RankRow>,
    pub chat: Vec<ChatLine>,
    pub np_log: Vec<NpEntry>,
    pub dirt: Vec<DirtView>,
    pub today: Today,
    pub ended: Option<EndView>,
    pub logo: String,
    pub hash: String,
}

fn chip(text: String, tone: &str) -> Chip {
    Chip {
        text,
        tone: tone.to_string(),
    }
}

/// Honest hint chips derived from the effects themselves.
pub fn hint(effects: &[Effect], who_name: &str) -> Vec<Chip> {
    let (mut fucks, mut susp, mut cash, mut np) = (0i64, 0i64, 0i64, 0i64);
    let mut out = vec![];
    let mut extra = vec![];
    for e in effects {
        fucks += e.fucks.unwrap_or(0) as i64;
        susp += e.susp.unwrap_or(0) as i64;
        cash += e.cash.unwrap_or(0);
        np += e.np.unwrap_or(0) as i64;
        if e.dirt == Some(true) {
            extra.push(chip(format!("dirt on {who_name}"), "good"));
        }
        if e.spawn.is_some() {
            extra.push(chip("???".into(), "neutral"));
        }
        if let (Some(c), Some(then)) = (e.chance, &e.then) {
            let inner = hint(then, who_name);
            let s: Vec<String> = inner.iter().map(|c| c.text.clone()).collect();
            let bad = inner.iter().any(|c| c.tone == "bad");
            extra.push(chip(
                format!("{c}%: {}", s.join(" ")),
                if bad { "bad" } else { "good" },
            ));
        }
    }
    if fucks != 0 {
        out.push(chip(
            format!("{fucks:+} fucks"),
            if fucks > 0 { "good" } else { "bad" },
        ));
    }
    if susp != 0 {
        out.push(chip(
            format!("{susp:+} susp"),
            if susp > 0 { "bad" } else { "good" },
        ));
    }
    if cash != 0 {
        out.push(chip(
            format!("{}${}", if cash > 0 { "+" } else { "-" }, cash.abs()),
            if cash > 0 { "good" } else { "bad" },
        ));
    }
    if np != 0 {
        out.push(chip(
            format!("{np:+} NP"),
            if np > 0 { "good" } else { "bad" },
        ));
    }
    out.extend(extra);
    out
}

fn clock(minute: u32) -> String {
    let total = START_HOUR * 60 + minute.min(DAY_MINUTES);
    format!("{:02}:{:02}", total / 60, total % 60)
}

impl Engine {
    pub fn title_for(&self, np: i32, rank: u32) -> String {
        let titles = &self.pack.quips.titles;
        let mut best = titles.first().map(|t| t.name.clone()).unwrap_or_default();
        let top = titles.iter().map(|t| t.np).max().unwrap_or(0);
        for t in titles {
            if np >= t.np {
                if t.np == top && rank != 1 {
                    continue; // the top title is only for #1
                }
                best = t.name.clone();
            }
        }
        best
    }

    fn board(&self) -> Vec<RankRow> {
        let mut rows: Vec<RankRow> = self
            .pack
            .rivals
            .iter()
            .zip(&self.s.rivals)
            .map(|(r, st)| RankRow {
                nick: r.nick.clone(),
                corp: r.corp.clone(),
                np: st.np,
                you: false,
                avatar: self
                    .pack
                    .art
                    .avatars
                    .get(&r.id)
                    .cloned()
                    .unwrap_or_default(),
            })
            .collect();
        rows.push(RankRow {
            nick: "you".into(),
            corp: self.pack.corp.name.clone(),
            np: self.s.np,
            you: true,
            avatar: String::new(),
        });
        // stable sort; ties favour the player
        rows.sort_by(|a, b| b.np.cmp(&a.np).then(b.you.cmp(&a.you)));
        rows
    }

    pub fn rank(&self) -> u32 {
        self.board()
            .iter()
            .position(|r| r.you)
            .map(|p| p as u32 + 1)
            .unwrap_or(1)
    }

    fn mood(&self) -> String {
        let q = &self.pack.quips;
        let pool = if self.s.fucks >= 60 {
            &q.mood_serene
        } else if self.s.fucks >= 30 {
            &q.mood_twitchy
        } else {
            &q.mood_critical
        };
        if pool.is_empty() {
            return String::new();
        }
        pool[(self.s.step as usize + self.s.day as usize) % pool.len()].clone()
    }

    pub fn view(&self) -> View {
        let pack = &self.pack;
        let rank = self.rank();
        let header = Header {
            corp: pack.corp.name.clone(),
            day: self.s.day,
            clock: clock(self.s.minute),
            minutes_left: DAY_MINUTES.saturating_sub(self.s.minute),
            cash: self.s.cash,
            retire_goal: RETIRE_GOAL,
            can_retire: self.s.cash >= RETIRE_GOAL && self.s.susp < RETIRE_MAX_SUSP,
            susp: self.s.susp,
            np: self.s.np,
            title: self.title_for(self.s.np, rank),
            rank,
            fucks: self.s.fucks,
            fucks_history: self.s.fucks_history.clone(),
            mood: self.mood(),
            toast: self.s.toast.clone(),
        };
        let tickets = self
            .s
            .queue
            .iter()
            .filter_map(|t| {
                let inc = pack.incident(&t.incident)?;
                let emp = pack.employee(&inc.who)?;
                let agitated = t.age > 0 || t.nth > 1 || inc.tier >= 3;
                let portrait = pack
                    .art
                    .portraits
                    .get(&inc.who)
                    .map(|p| {
                        if agitated && !p.agitated.is_empty() {
                            p.agitated.clone()
                        } else {
                            p.calm.clone()
                        }
                    })
                    .unwrap_or_default();
                let dirt = self
                    .s
                    .dirt
                    .iter()
                    .find(|d| d.who == inc.who)
                    .map(|d| DirtOffer {
                        name: emp.name.clone(),
                        label: d.label.clone(),
                    });
                Some(TicketView {
                    id: t.id,
                    title: inc.title.clone(),
                    who_id: emp.id.clone(),
                    who_name: emp.name.clone(),
                    who_role: emp.role.clone(),
                    nth: t.nth,
                    age: t.age,
                    quote: inc.quote.clone(),
                    details: inc.details.clone(),
                    agitated,
                    portrait,
                    options: inc
                        .options
                        .iter()
                        .enumerate()
                        .map(|(i, o)| OptionView {
                            n: i as u8,
                            text: o.text.clone(),
                            hint: hint(&o.effects, &emp.name),
                            kind: o.kind,
                            temptation: o.temptation,
                        })
                        .collect(),
                    dirt,
                })
            })
            .collect();
        let schemes = pack
            .schemes
            .iter()
            .map(|s| SchemeView {
                id: s.id.clone(),
                name: s.name.clone(),
                desc: s.desc.clone(),
                minutes: s.minutes,
                hint: vec![
                    chip(format!("+${}-{}", s.cash[0], s.cash[1]), "good"),
                    chip(format!("+{} susp", s.susp), "bad"),
                    chip(format!("{}% audit", s.audit_chance), "bad"),
                ],
                available: !self.s.schemes_today.contains(&s.id),
            })
            .collect();
        let stories = self
            .s
            .stories
            .iter()
            .map(|s| StoryView {
                id: s.id,
                text: s.text.clone(),
                value: s.value,
                risky: s.risky,
            })
            .collect();
        let dirt = self
            .s
            .dirt
            .iter()
            .map(|d| DirtView {
                name: pack
                    .employee(&d.who)
                    .map(|e| e.name.clone())
                    .unwrap_or_default(),
                label: d.label.clone(),
            })
            .collect();
        let ended = self.s.ended.as_ref().map(|e| self.end_view(e, rank));
        View {
            header,
            tickets,
            schemes,
            stories,
            board: self.board(),
            chat: self.s.chat.clone(),
            np_log: self.s.np_log.clone(),
            dirt,
            today: Today {
                rants: self.s.rants_today,
                coffees: self.s.coffees_today,
                boasts: self.s.boasts_today,
            },
            ended,
            logo: pack.art.logo.clone(),
            hash: self.state_hash(),
        }
    }

    fn end_view(&self, e: &Ended, rank: u32) -> EndView {
        let ending = self.pack.endings.iter().find(|x| x.id == e.ending);
        let fmt = |s: &str| {
            s.replace("{cause}", &e.cause)
                .replace("{days}", &self.s.day.to_string())
                .replace("{corp}", &self.pack.corp.name)
                .replace("{cash}", &format!("${}", self.s.cash))
                .replace("{np}", &self.s.np.to_string())
        };
        let title = self.title_for(self.s.np, rank);
        let head = match e.kind {
            EndKind::Meltdown => "MELTDOWN",
            EndKind::Fired => "FIRED",
            EndKind::Retired => "RETIRED (LEGEND)",
            EndKind::Ran => "TOOK THE MONEY AND RAN",
        };
        let share = format!(
            "FFS! {head} on day {} at {} | ${} | NP {} ({title}, #{rank}) | Cause: {} | seed {}",
            self.s.day, self.pack.corp.name, self.s.cash, self.s.np, e.cause, self.s.seed
        );
        EndView {
            kind: e.kind,
            title: ending
                .map(|x| x.title.clone())
                .unwrap_or_else(|| head.to_string()),
            lines: ending
                .map(|x| x.lines.iter().map(|l| fmt(l)).collect())
                .unwrap_or_default(),
            cause: e.cause.clone(),
            days: self.s.day,
            cash: self.s.cash,
            np: self.s.np,
            rank,
            nick_title: title,
            share,
        }
    }
}
