//! Content pack types, loading and validation for FFS!.
//!
//! The MVP pack is embedded in the binary (`Pack::base()`); the format is documented in
//! `content/packs/mvp/README.md`. No IO happens here at runtime.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("parse error in {file}: {msg}")]
    Parse { file: &'static str, msg: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub role: String,
    pub rank: u8,
    #[serde(default)]
    pub voice: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Corp {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub industry: String,
    #[serde(rename = "employee")]
    pub employees: Vec<Employee>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, serde::Serialize, ts_rs::TS,
)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum OptionKind {
    Professional,
    Malicious,
    Profitable,
    Vent,
    Dodge,
    Leverage,
}

/// One effect. Exactly one of the value fields is set, or `chance` + `then`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Effect {
    pub fucks: Option<i32>,
    pub susp: Option<i32>,
    pub cash: Option<i64>,
    pub np: Option<i32>,
    pub dirt: Option<bool>,
    pub spawn: Option<String>,
    pub chance: Option<u32>,
    pub then: Option<Vec<Effect>>,
}

impl Effect {
    fn kinds_set(&self) -> usize {
        [
            self.fucks.is_some(),
            self.susp.is_some(),
            self.cash.is_some(),
            self.np.is_some(),
            self.dirt.is_some(),
            self.spawn.is_some(),
            self.chance.is_some() || self.then.is_some(),
        ]
        .iter()
        .filter(|b| **b)
        .count()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct IncidentOption {
    pub id: String,
    pub text: String,
    pub kind: OptionKind,
    #[serde(default)]
    pub temptation: bool,
    pub minutes: Option<u32>,
    #[serde(default)]
    pub result: String,
    pub story: Option<String>,
    pub story_value: Option<u8>,
    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Incident {
    pub id: String,
    pub title: String,
    pub tier: u8,
    pub who: String,
    pub quote: String,
    #[serde(default)]
    pub details: String,
    #[serde(default)]
    pub cause: String,
    pub chain: Option<String>,
    pub step: Option<u32>,
    #[serde(default)]
    pub system: bool,
    #[serde(rename = "option")]
    pub options: Vec<IncidentOption>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scheme {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub minutes: u32,
    pub cash: [i64; 2],
    pub susp: i32,
    pub audit_chance: u32,
    pub story: String,
    pub story_value: u8,
    /// Percent chance that a run of this scheme hits big.
    #[serde(default)]
    pub jackpot_chance: u32,
    /// Cash multiplier range (inclusive) on a jackpot.
    #[serde(default = "one_one")]
    pub jackpot_mult: [i64; 2],
    /// Extra suspicion on a jackpot (big money gets noticed).
    #[serde(default)]
    pub jackpot_susp: i32,
    /// Toast shown on a jackpot.
    #[serde(default)]
    pub jackpot_line: String,
    /// First day the scheme can be run (default 1).
    #[serde(default)]
    pub min_day: u32,
}

fn one_one() -> [i64; 2] {
    [1, 1]
}

#[derive(Debug, Clone, Deserialize)]
pub struct Boast {
    pub text: String,
    pub np: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rival {
    pub id: String,
    pub nick: String,
    pub corp: String,
    pub ego: u8,
    pub np_start: [i32; 2],
    pub np_per_day: [i32; 2],
    pub idle: Vec<String>,
    pub boast: Vec<Boast>,
    pub react_high: Vec<String>,
    pub react_even: Vec<String>,
    pub react_low: Vec<String>,
    pub mock_help: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dirt {
    pub who: String,
    pub label: String,
    pub name: Option<String>,
    #[serde(rename = "use")]
    pub use_line: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ending {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Title {
    pub np: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Quips {
    #[serde(default)]
    pub day_end: Vec<String>,
    /// Toast openers for "cover your tracks".
    #[serde(default)]
    pub cover: Vec<String>,
    #[serde(default)]
    pub mood_serene: Vec<String>,
    #[serde(default)]
    pub mood_twitchy: Vec<String>,
    #[serde(default)]
    pub mood_critical: Vec<String>,
    #[serde(default)]
    pub titles: Vec<Title>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Portrait {
    #[serde(default)]
    pub calm: String,
    #[serde(default)]
    pub agitated: String,
}

#[derive(Debug, Clone, Default)]
pub struct Art {
    pub portraits: BTreeMap<String, Portrait>,
    pub avatars: BTreeMap<String, String>,
    pub logo: String,
}

#[derive(Debug, Clone)]
pub struct Pack {
    pub corp: Corp,
    pub incidents: Vec<Incident>,
    pub schemes: Vec<Scheme>,
    pub rivals: Vec<Rival>,
    pub dirt: Vec<Dirt>,
    pub endings: Vec<Ending>,
    pub quips: Quips,
    pub art: Art,
}

#[derive(Deserialize)]
struct IncidentFile {
    #[serde(default)]
    incident: Vec<Incident>,
}
#[derive(Deserialize)]
struct SchemeFile {
    scheme: Vec<Scheme>,
}
#[derive(Deserialize)]
struct RivalFile {
    rival: Vec<Rival>,
}
#[derive(Deserialize)]
struct DirtFile {
    dirt: Vec<Dirt>,
}
#[derive(Deserialize)]
struct EndingFile {
    ending: Vec<Ending>,
}
#[derive(Deserialize)]
struct AvatarArt {
    art: String,
}
#[derive(Deserialize, Default)]
struct ArtFile {
    #[serde(default)]
    portrait: BTreeMap<String, Portrait>,
    #[serde(default)]
    avatar: BTreeMap<String, AvatarArt>,
    #[serde(default)]
    logo: Option<toml::Value>,
}

fn parse<T: for<'de> Deserialize<'de>>(file: &'static str, src: &str) -> Result<T, ContentError> {
    toml::from_str(src).map_err(|e| ContentError::Parse {
        file,
        msg: e.to_string(),
    })
}

fn logo_text(v: Option<toml::Value>) -> String {
    match v {
        Some(toml::Value::String(s)) => s,
        Some(toml::Value::Table(t)) => {
            let art = t
                .get("art")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim_end()
                .to_string();
            match t.get("tagline").and_then(|v| v.as_str()) {
                Some(tag) => format!("{art}\n\n{tag}"),
                None => art,
            }
        }
        _ => String::new(),
    }
}

impl Pack {
    /// The embedded base MVP pack.
    pub fn base() -> Result<Pack, ContentError> {
        let corp: Corp = parse(
            "corp.toml",
            include_str!("../../../content/packs/mvp/corp.toml"),
        )?;
        let mut incidents = Vec::new();
        for (name, src) in [
            (
                "incidents/t1.toml",
                include_str!("../../../content/packs/mvp/incidents/t1.toml"),
            ),
            (
                "incidents/t2.toml",
                include_str!("../../../content/packs/mvp/incidents/t2.toml"),
            ),
            (
                "incidents/t3.toml",
                include_str!("../../../content/packs/mvp/incidents/t3.toml"),
            ),
            (
                "incidents/system.toml",
                include_str!("../../../content/packs/mvp/incidents/system.toml"),
            ),
        ] {
            let f: IncidentFile = parse(name, src)?;
            incidents.extend(f.incident);
        }
        let schemes: SchemeFile = parse(
            "schemes.toml",
            include_str!("../../../content/packs/mvp/schemes.toml"),
        )?;
        let rivals: RivalFile = parse(
            "rivals.toml",
            include_str!("../../../content/packs/mvp/rivals.toml"),
        )?;
        let dirt: DirtFile = parse(
            "dirt.toml",
            include_str!("../../../content/packs/mvp/dirt.toml"),
        )?;
        let endings: EndingFile = parse(
            "scenes.toml",
            include_str!("../../../content/packs/mvp/scenes.toml"),
        )?;
        let quips: Quips = parse(
            "quips.toml",
            include_str!("../../../content/packs/mvp/quips.toml"),
        )?;
        let art: ArtFile = parse(
            "art.toml",
            include_str!("../../../content/packs/mvp/art.toml"),
        )?;
        let mut dirt = dirt.dirt;
        for d in &mut dirt {
            if d.name.is_none() {
                d.name = corp
                    .employees
                    .iter()
                    .find(|e| e.id == d.who)
                    .map(|e| e.name.clone());
            }
        }
        Ok(Pack {
            corp,
            incidents,
            schemes: schemes.scheme,
            rivals: rivals.rival,
            dirt,
            endings: endings.ending,
            quips,
            art: Art {
                portraits: art.portrait,
                avatars: art.avatar.into_iter().map(|(k, v)| (k, v.art)).collect(),
                logo: logo_text(art.logo),
            },
        })
    }

    pub fn employee(&self, id: &str) -> Option<&Employee> {
        self.corp.employees.iter().find(|e| e.id == id)
    }

    pub fn incident(&self, id: &str) -> Option<&Incident> {
        self.incidents.iter().find(|i| i.id == id)
    }

    /// Validate the pack. Returns human-readable problems; empty means OK.
    pub fn validate(&self) -> Vec<String> {
        let mut errs = Vec::new();
        let emp_ids: BTreeSet<&str> = self.corp.employees.iter().map(|e| e.id.as_str()).collect();
        let inc_ids: BTreeSet<&str> = self.incidents.iter().map(|i| i.id.as_str()).collect();
        if inc_ids.len() != self.incidents.len() {
            errs.push("duplicate incident ids".into());
        }
        for req in ["sys_audit_scare", "sys_hr_meeting", "sys_auditor_visit"] {
            if !inc_ids.contains(req) {
                errs.push(format!("missing required system incident {req}"));
            }
        }
        let mut chains: BTreeMap<&str, Vec<u32>> = BTreeMap::new();
        for i in &self.incidents {
            let at = format!("incident {}", i.id);
            if !emp_ids.contains(i.who.as_str()) {
                errs.push(format!("{at}: unknown employee {}", i.who));
            }
            if !(1..=3).contains(&i.tier) {
                errs.push(format!("{at}: tier must be 1-3"));
            }
            if i.quote.chars().count() > 240 {
                errs.push(format!("{at}: quote too long"));
            }
            if !i.system && i.cause.is_empty() {
                errs.push(format!("{at}: missing cause"));
            }
            if !(3..=4).contains(&i.options.len()) {
                errs.push(format!("{at}: needs 3-4 options"));
            }
            let kinds: BTreeSet<OptionKind> = i.options.iter().map(|o| o.kind).collect();
            if !kinds.contains(&OptionKind::Professional) && !i.system {
                errs.push(format!("{at}: no professional option"));
            }
            let mut oids = BTreeSet::new();
            for o in &i.options {
                if !oids.insert(o.id.as_str()) {
                    errs.push(format!("{at}: duplicate option id {}", o.id));
                }
                if o.text.chars().count() > 62 {
                    errs.push(format!("{at}/{}: text too long", o.id));
                }
                if o.story.is_some() != o.story_value.is_some() {
                    errs.push(format!("{at}/{}: story and story_value go together", o.id));
                }
                for e in &o.effects {
                    self.check_effect(&at, e, &inc_ids, &mut errs);
                }
            }
            if let Some(c) = &i.chain {
                chains
                    .entry(c.as_str())
                    .or_default()
                    .push(i.step.unwrap_or(0));
            }
        }
        for (c, mut steps) in chains {
            steps.sort_unstable();
            let want: Vec<u32> = (0..steps.len() as u32).collect();
            if steps != want {
                errs.push(format!(
                    "chain {c}: steps must be contiguous from 0, got {steps:?}"
                ));
            }
        }
        for d in &self.dirt {
            if !emp_ids.contains(d.who.as_str()) {
                errs.push(format!("dirt: unknown employee {}", d.who));
            }
        }
        for sc in &self.schemes {
            if sc.jackpot_chance > 100 {
                errs.push(format!("scheme {}: jackpot_chance must be 0-100", sc.id));
            }
            if sc.jackpot_chance > 0
                && (sc.jackpot_mult[0] < 2
                    || sc.jackpot_mult[1] < sc.jackpot_mult[0]
                    || sc.jackpot_line.is_empty())
            {
                errs.push(format!(
                    "scheme {}: a jackpot needs jackpot_mult >= 2 and a jackpot_line",
                    sc.id
                ));
            }
        }
        for r in &self.rivals {
            // enough canned lines that a ten-minute run rarely repeats one
            if r.idle.len() < 20
                || r.boast.len() < 16
                || r.react_high.len() < 8
                || r.react_even.len() < 8
                || r.react_low.len() < 8
                || r.mock_help.len() < 6
            {
                errs.push(format!(
                    "rival {}: too few lines (need idle 20, boast 16, each react 8, mock_help 6)",
                    r.id
                ));
            }
            if !(1..=3).contains(&r.ego) {
                errs.push(format!("rival {}: ego must be 1-3", r.id));
            }
        }
        for k in ["meltdown", "fired", "retire", "ran"] {
            if !self.endings.iter().any(|e| e.kind == k) {
                errs.push(format!("no ending of kind {k}"));
            }
        }
        if self.quips.cover.len() < 6 {
            errs.push("quips: need at least 6 cover lines".into());
        }
        if self.quips.titles.is_empty() {
            errs.push("quips: titles missing".into());
        }
        errs
    }

    fn check_effect(&self, at: &str, e: &Effect, inc_ids: &BTreeSet<&str>, errs: &mut Vec<String>) {
        if e.kinds_set() != 1 {
            errs.push(format!("{at}: effect must set exactly one thing: {e:?}"));
        }
        if let Some(s) = &e.spawn {
            if !inc_ids.contains(s.as_str()) {
                errs.push(format!("{at}: spawn target {s} not found"));
            }
        }
        if e.chance.is_some() != e.then.is_some() {
            errs.push(format!("{at}: chance and then go together"));
        }
        if let Some(c) = e.chance {
            if c == 0 || c > 100 {
                errs.push(format!("{at}: chance must be 1-100"));
            }
        }
        if let Some(t) = &e.then {
            for x in t {
                if x.chance.is_some() {
                    errs.push(format!("{at}: nested chance not allowed"));
                }
                self.check_effect(at, x, inc_ids, errs);
            }
        }
    }
}
