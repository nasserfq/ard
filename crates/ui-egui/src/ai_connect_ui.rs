//! Help › Connect to AI Apps (also File › Connect to AI Apps, File › About and Help in the menu)
//! («ربط مع الذكاء الاصطناعي»): connects عرض's MCP server (`ard-cli mcp`, shipped beside the app)
//! to the AI apps on this computer with one button each, by writing their own settings (the
//! `ai-connect` crate). Other MCP apps get the settings to paste. Desktop only: the web build has no MCP server and no settings files.

use crate::SlideApp;

#[derive(Default)]
pub(crate) struct Runtime {
    pub(crate) open: bool,
    rows: Vec<(ai_connect::Client, ai_connect::State)>,
    /// The last action's result: text and whether it went wrong.
    message: Option<(String, bool)>,
}

/// The MCP server beside the running app.
fn server() -> Option<ai_connect::Server> {
    let exe = std::env::current_exe().ok()?;
    let command = ai_connect::find_server(&exe, &["ard-cli", "deckcraft-cli"])?;
    Some(ai_connect::Server { key: "ard".into(), command, args: vec!["mcp".into()] })
}

fn refresh(rt: &mut Runtime) {
    rt.rows.clear();
    if let (Some(dirs), Some(server)) = (ai_connect::Dirs::system(), server()) {
        rt.rows = ai_connect::Client::ALL.into_iter().map(|c| (c, ai_connect::state(c, &dirs, &server))).collect();
    }
}

/// Open the window (the `app.connectAi` command without `client`).
pub(crate) fn open(app: &mut SlideApp) {
    app.connect_ai.open = true;
    app.connect_ai.message = None;
    refresh(&mut app.connect_ai);
}

/// Connect or disconnect `client` without the window (`app.connectAi` with `client` and
/// `disconnect`): what the control channel, tests and agents use.
pub(crate) fn run(client: &str, disconnect: bool) -> Result<serde_json::Value, String> {
    let client = ai_connect::Client::from_id(client).ok_or_else(|| {
        let ids: Vec<&str> = ai_connect::Client::ALL.iter().map(|c| c.id()).collect();
        format!("unknown AI app {client:?}; one of: {}", ids.join(", "))
    })?;
    let dirs = ai_connect::Dirs::system().ok_or("the home folder is unknown")?;
    let server = server().ok_or("the MCP server (ard-cli) is not beside the app")?;
    if disconnect {
        ai_connect::disconnect(client, &dirs, &server).map_err(|e| e.to_string())?;
        return Ok(serde_json::json!({ "client": client.id(), "state": "notConnected" }));
    }
    match ai_connect::connect(client, &dirs, &server).map_err(|e| e.to_string())? {
        ai_connect::Done::Saved => Ok(serde_json::json!({ "client": client.id(), "state": "connected" })),
        ai_connect::Done::OpenLink(link) => Ok(serde_json::json!({ "client": client.id(), "link": link })),
    }
}

fn state_label(s: &ai_connect::State) -> &'static str {
    use ai_connect::State;
    match s {
        State::NotInstalled => "Not installed",
        State::NotConnected => "Not connected",
        State::Connected => "Connected ✓",
        State::Outdated => "Needs updating",
        State::Unreadable(_) => "Its settings can't be read",
    }
}

pub(crate) fn show(app: &mut SlideApp, ctx: &egui::Context) {
    if !app.connect_ai.open {
        return;
    }
    let mut open = true;
    let mut close = false;
    let shown = egui::Window::new(tl!("Connect to AI Apps"))
        .id(egui::Id::unique("connect-ai-window"))
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .default_width(440.0)
        .show(ctx, |ui| {
            ui.set_max_width(440.0);
            body(app, ui);
            ui.add_space(8.0);
            if ui.button(tl!("Close")).clicked() {
                close = true;
            }
        });
    // Mirrored as a whole in a right-to-left interface, like every dialog.
    if let Some(shown) = shown {
        crate::mirror::overlay(ctx, shown.response.layer_id);
    }
    if !open || close {
        app.connect_ai.open = false;
    }
}

fn body(app: &mut SlideApp, ui: &mut egui::Ui) {
    use ai_connect::State;
    ui.label(tl!("Connect Ard to an AI app so it can build, write and design your presentations when you ask it to. Press Connect next to the app, then restart it."));
    ui.add_space(6.0);
    let Some(server) = server() else {
        ui.colored_label(ui.visuals().error_fg_color, tl!("The MCP server (ard-cli) isn't next to Ard. Download Ard again."));
        return;
    };
    if ai_connect::temporary_location(&server.command) {
        ui.colored_label(
            ui.visuals().warn_fg_color,
            tl!("Move Ard to the Applications folder and open it from there first, or the connection stops working once it closes."),
        );
        ui.add_space(4.0);
    }
    let Some(dirs) = ai_connect::Dirs::system() else { return };
    let mut action: Option<(ai_connect::Client, bool)> = None;
    let installed: Vec<_> = app.connect_ai.rows.iter().filter(|(_, s)| *s != State::NotInstalled).cloned().collect();
    if installed.is_empty() {
        ui.label(tl!("No AI app we know of is installed here. Install one, such as Claude, then open this window again."));
    } else {
        egui::Grid::new("connect_ai_grid").num_columns(3).spacing([16.0, 8.0]).show(ui, |ui| {
            for (client, state) in &installed {
                ui.label(egui::RichText::new(client.name()).strong());
                let label = ui.label(tl!(state_label(state)));
                if let State::Unreadable(why) = state {
                    label.on_hover_text(why);
                }
                let button = match state {
                    State::NotConnected => Some(("Connect", false)),
                    State::Outdated => Some(("Update", false)),
                    State::Connected => Some(("Disconnect", true)),
                    State::NotInstalled | State::Unreadable(_) => None,
                };
                match button {
                    Some((text, disconnect)) => {
                        if ui.button(tl!(text)).clicked() {
                            action = Some((*client, disconnect));
                        }
                    }
                    None => {
                        ui.label("");
                    }
                }
                ui.end_row();
            }
        });
    }
    let missing: Vec<&str> = app.connect_ai.rows.iter().filter(|(_, s)| *s == State::NotInstalled).map(|(c, _)| c.name()).collect();
    if !missing.is_empty() {
        ui.add_space(4.0);
        let list = missing.join(if crate::i18n::current().code() == "ar" { "، " } else { ", " });
        ui.label(egui::RichText::new(format!("{} {list}", tl!("Not installed here:"))).small().weak());
    }
    if let Some((client, disconnect)) = action {
        let result = if disconnect {
            ai_connect::disconnect(client, &dirs, &server).map(|()| tl!("Disconnected.").to_string())
        } else {
            ai_connect::connect(client, &dirs, &server).map(|done| match done {
                ai_connect::Done::Saved if client.needs_restart() => tl!("Connected. Restart {app}.").replace("{app}", client.name()),
                ai_connect::Done::Saved => tl!("Connected.").to_string(),
                ai_connect::Done::OpenLink(link) => {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(link));
                    tl!("Confirm the install in {app}.").replace("{app}", client.name())
                }
            })
        };
        app.connect_ai.message = Some(match result {
            Ok(text) => (text, false),
            Err(e) => {
                log::warn!("connecting {}: {e}", client.name());
                (format!("{} {e}", tl!("Couldn't connect:")), true)
            }
        });
        refresh(&mut app.connect_ai);
    }
    if let Some((text, bad)) = &app.connect_ai.message {
        ui.add_space(6.0);
        if *bad {
            ui.colored_label(ui.visuals().error_fg_color, text);
        } else {
            ui.label(egui::RichText::new(text).strong());
        }
    }
    ui.add_space(8.0);
    ui.separator();
    ui.label(tl!("Another MCP app? Copy the settings and paste them into its MCP servers settings."));
    if ui.button(tl!("Copy Settings")).clicked() {
        ui.ctx().copy_text(server.snippet());
        app.connect_ai.message = Some((tl!("Settings copied.").to_string(), false));
    }
}
