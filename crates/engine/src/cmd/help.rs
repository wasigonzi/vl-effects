//! Help menu: community and project links (the frontend opens `Event::OpenUrl`).

use serde_json::{Value, json};

use super::{CommandSpec, always, str_p};
use crate::links;
use crate::{Event, Result, Session, cmd};

fn open(s: &mut Session, url: String) -> Result<Value> {
    s.events.push(Event::OpenUrl(url.clone()));
    Ok(json!({"url": url}))
}

fn sibling(s: &mut Session, p: &Value) -> Result<Value> {
    let slug = str_p(p, "app").unwrap_or("vl-studios").to_ascii_lowercase();
    open(s, links::page_of(&slug))
}

/// Help ▸ VL Effects Help / Scripting Help / Expression Reference / Effect Reference: VL Effects'
/// page on visualabstudios.com.
fn docs(s: &mut Session, _p: &Value) -> Result<Value> {
    open(s, links::APP_PAGE.into())
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!("help.docs", "VL Effects Help...", [], Some("F1"), "{page?: help|scripting|expressions|effects|agents}", always, docs),
        cmd!("help.discord", "Visit Visualab Studios...", ["Help"], None, "{}", always, |s, _| open(s, links::DISCORD.into())),
        cmd!("help.website", "Visualab Studios Website", ["Help"], None, "{}", always, |s, _| open(s, links::WEBSITE.into())),
        cmd!("help.appPage", "VL Effects Home Page", ["Help"], None, "{}", always, |s, _| open(s, links::APP_PAGE.into())),
        cmd!("help.onlineTutorials", "Online Tutorials...", ["Help"], None, "{}", always, |s, _| open(s, links::APP_PAGE.into())),
        cmd!("help.inAppTutorials", "In-App Tutorials...", ["Help"], None, "{} → the Home screen's Learn tab", always, |s, p| super::frontend(
            s,
            "help.inAppTutorials",
            p
        )),
        cmd!("help.reportIssue", "Provide Feedback...", ["Help"], None, "{}", always, |s, _| open(s, links::CONTACT.into())),
        cmd!("help.sibling", "Other VL Tools", [], None, "{app: vl-studios|vl-vector|vl-films}", always, sibling),
    ]
}
