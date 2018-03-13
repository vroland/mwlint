extern crate mediawiki_parser;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate colored;
#[macro_use]
extern crate lazy_static;


/// Provides linter result definitions.
mod lint;
/// Structures for configuration of linter behaviour.
mod settings;
/// Various helper functions.
#[macro_use]
mod utils;
/// Data structures for defining rules.
#[macro_use]
mod rule;
mod template_spec;

/// common imports for rules.
mod preamble {
    pub use lint::{Lint, LintKind, Severity, Example};
    pub use rule::*;
    pub use mediawiki_parser::*;
    pub use settings::{Settings, RuleMeta};
    pub use std::io;
}

/// The checking functions themselves.
mod rules;

pub use settings::{Settings};
pub use rule::{Rule, Checkable};
pub use lint::{Example, Lint, Severity};
pub use rules::*;
