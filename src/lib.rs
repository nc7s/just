#![deny(clippy::all, clippy::pedantic)]
#![allow(
  clippy::enum_glob_use,
  clippy::let_underscore_untyped,
  clippy::needless_pass_by_value,
  clippy::similar_names,
  clippy::struct_excessive_bools,
  clippy::struct_field_names,
  clippy::too_many_arguments,
  clippy::too_many_lines,
  clippy::unnecessary_wraps,
  clippy::wildcard_imports,
  overlapping_range_endpoints
)]

pub(crate) use {
  crate::{
    alias::Alias, analyzer::Analyzer, argument_parser::ArgumentParser, assignment::Assignment,
    assignment_resolver::AssignmentResolver, ast::Ast, attribute::Attribute, binding::Binding,
    color::Color, color_display::ColorDisplay, command_ext::CommandExt, compilation::Compilation,
    compile_error::CompileError, compile_error_kind::CompileErrorKind, compiler::Compiler,
    condition::Condition, conditional_operator::ConditionalOperator, config::Config,
    config_error::ConfigError, constants::constants, count::Count, delimiter::Delimiter,
    dependency::Dependency, dump_format::DumpFormat, enclosure::Enclosure, error::Error,
    evaluator::Evaluator, execution_context::ExecutionContext, expression::Expression,
    fragment::Fragment, function::Function, interrupt_guard::InterruptGuard,
    interrupt_handler::InterruptHandler, item::Item, justfile::Justfile, keyed::Keyed,
    keyword::Keyword, lexer::Lexer, line::Line, list::List, load_dotenv::load_dotenv,
    loader::Loader, module_path::ModulePath, name::Name, namepath::Namepath, ordinal::Ordinal,
    output::output, output_error::OutputError, parameter::Parameter, parameter_kind::ParameterKind,
    parser::Parser, platform::Platform, platform_interface::PlatformInterface, position::Position,
    positional::Positional, ran::Ran, range_ext::RangeExt, recipe::Recipe,
    recipe_resolver::RecipeResolver, recipe_signature::RecipeSignature, scope::Scope,
    search::Search, search_config::SearchConfig, search_error::SearchError, set::Set,
    setting::Setting, settings::Settings, shebang::Shebang, shell::Shell,
    show_whitespace::ShowWhitespace, source::Source, string_kind::StringKind,
    string_literal::StringLiteral, subcommand::Subcommand, suggestion::Suggestion, table::Table,
    thunk::Thunk, token::Token, token_kind::TokenKind, unresolved_dependency::UnresolvedDependency,
    unresolved_recipe::UnresolvedRecipe, use_color::UseColor, variables::Variables,
    verbosity::Verbosity, warning::Warning,
  },
  camino::Utf8Path,
  derivative::Derivative,
  edit_distance::edit_distance,
  lexiclean::Lexiclean,
  libc::EXIT_FAILURE,
  log::{info, warn},
  regex::Regex,
  serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize, Serializer,
  },
  snafu::{ResultExt, Snafu},
  std::{
    borrow::Cow,
    cmp,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    env,
    ffi::OsString,
    fmt::{self, Debug, Display, Formatter},
    fs,
    io::{self, Read, Seek, Write},
    iter::{self, FromIterator},
    mem,
    ops::Deref,
    ops::{Index, Range, RangeInclusive},
    path::{self, Path, PathBuf},
    process::{self, Command, ExitStatus, Stdio},
    rc::Rc,
    str::{self, Chars},
    sync::{Mutex, MutexGuard, OnceLock},
    vec,
  },
  strum::{Display, EnumDiscriminants, EnumString, IntoStaticStr},
  tempfile::tempfile,
  typed_arena::Arena,
  unicode_width::{UnicodeWidthChar, UnicodeWidthStr},
};

#[cfg(test)]
pub(crate) use crate::{node::Node, tree::Tree};

pub use crate::run::run;

// Used in integration tests.
#[doc(hidden)]
pub use unindent::unindent;

type CompileResult<'a, T = ()> = Result<T, CompileError<'a>>;
type ConfigResult<T> = Result<T, ConfigError>;
type FunctionResult = Result<String, String>;
type RunResult<'a, T = ()> = Result<T, Error<'a>>;
type SearchResult<T> = Result<T, SearchError>;

#[cfg(test)]
#[macro_use]
pub mod testing;

#[cfg(test)]
#[macro_use]
pub mod tree;

#[cfg(test)]
pub mod node;

#[cfg(fuzzing)]
pub mod fuzzing;

// Used by Janus, https://github.com/casey/janus, a tool
// that analyses all public justfiles on GitHub to avoid
// breaking changes.
#[doc(hidden)]
pub mod summary;

mod alias;
mod analyzer;
mod argument_parser;
mod assignment;
mod assignment_resolver;
mod ast;
mod attribute;
mod binding;
mod color;
mod color_display;
mod command_ext;
mod compilation;
mod compile_error;
mod compile_error_kind;
mod compiler;
mod completions;
mod condition;
mod conditional_operator;
mod config;
mod config_error;
mod constants;
mod count;
mod delimiter;
mod dependency;
mod dump_format;
mod enclosure;
mod error;
mod evaluator;
mod execution_context;
mod expression;
mod fragment;
mod function;
mod interrupt_guard;
mod interrupt_handler;
mod item;
mod justfile;
mod keyed;
mod keyword;
mod lexer;
mod line;
mod list;
mod load_dotenv;
mod loader;
mod module_path;
mod name;
mod namepath;
mod ordinal;
mod output;
mod output_error;
mod parameter;
mod parameter_kind;
mod parser;
mod platform;
mod platform_interface;
mod position;
mod positional;
mod ran;
mod range_ext;
mod recipe;
mod recipe_resolver;
mod recipe_signature;
mod run;
mod scope;
mod search;
mod search_config;
mod search_error;
mod set;
mod setting;
mod settings;
mod shebang;
mod shell;
mod show_whitespace;
mod source;
mod string_kind;
mod string_literal;
mod subcommand;
mod suggestion;
mod table;
mod thunk;
mod token;
mod token_kind;
mod unindent;
mod unresolved_dependency;
mod unresolved_recipe;
mod use_color;
mod variables;
mod verbosity;
mod warning;
