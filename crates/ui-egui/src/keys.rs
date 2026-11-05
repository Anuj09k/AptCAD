//! The keymap: what a two-letter name typed at the command line starts.
//!
//! Aptimizer's keymap (the default) adds the keys Aptimizer Studio's building mode has for the
//! tools both modes share, so a key does the same in both: MV Move, OF Offset, DE Erase (Delete),
//! XP Explode and JO Join, beside the ones that already agree (CO, RO, MI, TR, AL, BR, MA, PU,
//! PR). Of these only MV was another command's alias (MVIEW, which its name still starts). Classic
//! keeps the aliases alone. The keymap changes only what is typed at the command line: scripts,
//! LISP and the JSON form read the aliases as they are, and an alias of the user's own
//! (ALIASEDIT) comes first in either.

use serde::{Deserialize, Serialize};

/// The keymaps.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Keymap {
    #[default]
    Aptimizer,
    Classic,
}

/// Aptimizer's keys the aliases don't have: each with the command it starts.
pub const SHARED: [(&str, &str); 5] = [("mv", "move"), ("of", "offset"), ("de", "erase"), ("xp", "explode"), ("jo", "join")];

impl Keymap {
    pub const ALL: [Keymap; 2] = [Keymap::Aptimizer, Keymap::Classic];

    pub fn as_str(self) -> &'static str {
        match self {
            Keymap::Aptimizer => "aptimizer",
            Keymap::Classic => "classic",
        }
    }

    pub fn parse(s: &str) -> Option<Keymap> {
        Keymap::ALL.into_iter().find(|k| k.as_str().eq_ignore_ascii_case(s.trim()))
    }

    /// The command `name`, typed at the command line, starts in this keymap where that isn't what
    /// the aliases say: an Aptimizer key the user hasn't made an alias of their own.
    pub fn command(self, name: &str) -> Option<&'static str> {
        if self == Keymap::Classic {
            return None;
        }
        let name = name.to_ascii_lowercase();
        if aptcad_engine::cmd::user_alias(&name).is_some() {
            return None;
        }
        SHARED.iter().find(|(key, _)| *key == name).map(|(_, id)| *id)
    }
}
