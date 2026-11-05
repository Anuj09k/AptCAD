//! The keymap: Aptimizer's command keys (the default) add MV, OF, DE, XP and JO as Aptimizer
//! Studio's building mode has them; Classic keeps the aliases alone. Only what is typed at the
//! command line changes: the engine, and so scripts and LISP, read the aliases as they are.

use aptcad_engine::Session;
use aptcad_ui_egui::keys::Keymap;
use aptcad_ui_egui::{CadApp, Services};
use serde_json::json;

fn app() -> CadApp {
    let mut app = CadApp::new(Session::new(), Services::default());
    app.cmdline("line 0,0 10,0 ");
    app
}

/// What the command line asks after `typed`, Escape after.
fn asks(app: &mut CadApp, typed: &str) -> String {
    let refused = app.cmdline(typed);
    let prompt = if app.session.prompting() { app.session.prompt_text() } else { format!("refused: {refused:?}") };
    let _ = app.session.input(aptcad_engine::Input::Cancel);
    prompt
}

#[test]
fn aptimizers_keys_start_the_building_modes_commands_and_classic_keeps_the_aliases() {
    let mut app = app();
    assert_eq!(app.ui.keymap, Keymap::Aptimizer, "the default");
    let mv = asks(&mut app, "mv");
    assert_eq!(mv, asks(&mut app, "move"), "MV is Move");
    assert_ne!(mv, asks(&mut app, "mview"), "not MVIEW, whose name still starts it");
    for (key, command) in [("of", "offset"), ("de", "erase"), ("xp", "explode"), ("jo", "join")] {
        assert_eq!(asks(&mut app, key), asks(&mut app, command), "{key}");
    }
    app.ui.keymap = Keymap::Classic;
    assert_eq!(asks(&mut app, "mv"), asks(&mut app, "mview"), "Classic's MV is MVIEW's alias");
    assert!(asks(&mut app, "xp").starts_with("refused"), "no such command or alias");
}

#[test]
fn the_engine_reads_the_aliases_as_they_are_whatever_the_keymap() {
    let mut app = app();
    let _ = app.session.cmdline("mv");
    let engine = if app.session.prompting() { app.session.prompt_text() } else { String::new() };
    let _ = app.session.input(aptcad_engine::Input::Cancel);
    assert_eq!(engine, {
        let _ = app.session.cmdline("mview");
        let p = if app.session.prompting() { app.session.prompt_text() } else { String::new() };
        let _ = app.session.input(aptcad_engine::Input::Cancel);
        p
    });
}

#[test]
fn options_sets_the_keymap_and_the_prefs_keep_it() {
    let mut app = app();
    app.run("options", json!({"keymap": "classic"}));
    assert_eq!(app.ui.keymap, Keymap::Classic);
    assert!(app.run("options", json!({"keymap": "qwerty"})).is_err());
    let mut again = CadApp::new(Session::new(), Services::default());
    again.load_prefs(&app.prefs_json());
    assert_eq!(again.ui.keymap, Keymap::Classic);
    app.run("options", json!({"reset": true}));
    assert_eq!(app.ui.keymap, Keymap::Aptimizer);
}
