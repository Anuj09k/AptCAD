//! The JSON form of a command that acts on objects, run with no `handle`/`handles` and nothing
//! selected, is an error that says so. It never reports success having done nothing (a model
//! answered "draw a row of 5 circles" with CIRCLE then ARRAYRECT: the new circle isn't selected,
//! so the array copied nothing yet reported success).

use aptcad_engine::Session;
use serde_json::{Value, json};

/// Every object in model space, to show a refused command changed nothing.
fn snapshot(s: &mut Session) -> Value {
    s.execute("entities", &json!({})).unwrap()
}

/// A drawing with one circle (not selected) and one line to path-array along.
fn drawing() -> (Session, String) {
    let mut s = Session::new();
    s.execute("circle", &json!({ "center": [0, 0], "radius": 100 })).unwrap();
    let path = s.execute("line", &json!({ "points": [[0, 0], [5000, 0]] })).unwrap()["handles"][0].as_str().unwrap().to_string();
    s.execute("select", &json!({ "handles": [] })).unwrap();
    assert!(s.selection().is_empty());
    (s, path)
}

/// `cmd` with `params` and nothing selected fails with `msg`, and so does an explicit empty
/// `handles`; neither changes the drawing.
fn refused(cmd: &str, params: Value, msg: &str) {
    let (mut s, path) = drawing();
    let mut params = params;
    if cmd == "arraypath" {
        params["path"] = json!(path);
    }
    let before = snapshot(&mut s);
    let mut empty = params.clone();
    empty["handles"] = json!([]);
    for p in [params, empty] {
        let e = s.execute(cmd, &p).expect_err(&format!("{cmd} {p} must fail with nothing to act on"));
        let e = e.to_string();
        assert!(e.contains(msg) && e.contains("give `handles` or select objects first"), "{cmd} {p}: {e}");
        assert_eq!(snapshot(&mut s), before, "{cmd} {p} must change nothing");
    }
}

const NO_ARRAY: &str = "No objects to array";
const NOTHING: &str = "Nothing selected";

#[test]
fn arrayrect_with_nothing_selected_is_an_error() {
    // The reported call, after CIRCLE.
    refused("arrayrect", json!({ "associative": false, "cols": 5, "rows": 1, "colSpacing": 1000, "rowSpacing": 0 }), NO_ARRAY);
    refused("arrayrect", json!({ "associative": true, "cols": 5, "rows": 1, "colSpacing": 1000 }), NO_ARRAY);
}

#[test]
fn arraypolar_with_nothing_selected_is_an_error() {
    refused("arraypolar", json!({ "associative": false, "center": [0, 0], "count": 6 }), NO_ARRAY);
    refused("arraypolar", json!({ "associative": true, "center": [0, 0], "count": 6 }), NO_ARRAY);
}

#[test]
fn arraypath_with_nothing_selected_is_an_error() {
    refused("arraypath", json!({ "count": 5 }), NO_ARRAY);
}

#[test]
fn array_with_nothing_selected_is_an_error() {
    for ty in ["rectangular", "polar"] {
        refused("array", json!({ "type": ty, "center": [0, 0], "associative": false }), NO_ARRAY);
    }
}

#[test]
fn classic_array_with_nothing_selected_is_an_error() {
    refused("-array", json!({ "rows": 1, "cols": 5, "colSpacing": 1000 }), NO_ARRAY);
    refused("-array", json!({ "type": "polar", "center": [0, 0], "count": 6 }), NO_ARRAY);
}

#[test]
fn erase_with_nothing_selected_is_an_error() {
    refused("erase", json!({}), NOTHING);
}

#[test]
fn move_with_nothing_selected_is_an_error() {
    refused("move", json!({ "delta": [10, 0] }), NOTHING);
}

#[test]
fn copy_with_nothing_selected_is_an_error() {
    refused("copy", json!({ "delta": [10, 0] }), NOTHING);
}

#[test]
fn rotate_with_nothing_selected_is_an_error() {
    refused("rotate", json!({ "base": [0, 0], "angle": 90 }), NOTHING);
}

#[test]
fn scale_with_nothing_selected_is_an_error() {
    refused("scale", json!({ "base": [0, 0], "factor": 2 }), NOTHING);
}

#[test]
fn mirror_with_nothing_selected_is_an_error() {
    refused("mirror", json!({ "p1": [0, 0], "p2": [0, 10] }), NOTHING);
}

/// The fix for the reported call: give the circle's handle, and five circles are made.
#[test]
fn arrayrect_with_the_new_circle_makes_the_row() {
    let mut s = Session::new();
    let c = s.execute("circle", &json!({ "center": [0, 0], "radius": 100 })).unwrap()["handle"].as_str().unwrap().to_string();
    let r =
        s.execute("arrayrect", &json!({ "handles": [c], "associative": false, "cols": 5, "rows": 1, "colSpacing": 1000, "rowSpacing": 0 })).unwrap();
    assert_eq!(r["created"], 4);
    let circles = s.execute("entities", &json!({ "type": "Circle" })).unwrap()["entities"].as_array().map(Vec::len);
    assert_eq!(circles, Some(5));
}
