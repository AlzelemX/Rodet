use std::borrow::Cow;

pub use rustc_error_messages::FluentArgs;
use rustc_error_messages::{
    DiagArgMap, DiagArgName, DiagArgValue, IntoDiagArg, langid, register_functions,
};
use tracing::{debug, trace};

use crate::fluent_bundle::FluentResource;
use crate::{DiagArg, DiagMessage, Style, fluent_bundle};

/// Convert diagnostic arguments (a rustc internal type that exists to implement
/// `Encodable`/`Decodable`) into `FluentArgs` which is necessary to perform formatting.
fn to_fluent_args<'iter>(iter: impl Iterator<Item = DiagArg<'iter>>) -> FluentArgs<'static> {
    let mut args = if let Some(size) = iter.size_hint().1 {
        FluentArgs::with_capacity(size)
    } else {
        FluentArgs::new()
    };

    for (k, v) in iter {
        args.set(k.clone(), v.clone());
    }

    args
}

/// Convert `DiagMessage`s to a string
pub fn format_diag_messages(messages: &[(DiagMessage, Style)], args: &DiagArgMap) -> String {
    messages.iter().map(|(m, _)| format_diag_message(m, args)).collect::<String>()
}

/// Convert a `DiagMessage` to a string
pub fn format_diag_message<'a>(message: &'a DiagMessage, args: &DiagArgMap) -> Cow<'a, str> {
    // RUST++: the message in the user's language (`RUSTPP_LANG`), see `i18n.rs`.
    match message {
        DiagMessage::Str(msg) => match crate::i18n::text(msg) {
            Some(translated) => Cow::Owned(translated),
            None => Cow::Borrowed(msg),
        },
        DiagMessage::Inline(msg) => crate::i18n::template(msg)
            .and_then(|translated| try_format_fluent_str(translated, args))
            .map(Cow::Owned)
            .unwrap_or_else(|| format_fluent_str(msg, args)),
    }
}

/// RUST++: formats a translated template; `None` if it can't be read (then
/// the English message is shown instead of failing).
fn try_format_fluent_str(message: &str, args: &DiagArgMap) -> Option<String> {
    const GENERATED_MSG_ID: &str = "generated_msg";
    let resource = FluentResource::try_new(format!("{GENERATED_MSG_ID} = {message}\n")).ok()?;
    let mut bundle = fluent_bundle::FluentBundle::new(vec![langid!("en-US")]);
    bundle.set_use_isolating(false);
    bundle.add_resource(resource).ok()?;
    register_functions(&mut bundle);
    let value = bundle.get_message(GENERATED_MSG_ID)?.value()?;
    // Words the compiler passes in ("function", "this scope"…) are translated
    // too, unless the message shows them as code or chooses between texts by them.
    let args: DiagArgMap = args
        .iter()
        .map(|(name, value)| {
            let value = match value {
                DiagArgValue::Str(s)
                    if !message.contains(&format!("`{{${name}}}`"))
                        && !message.contains(&format!("{{${name} ->"))
                        && !message.contains(&format!("{{ ${name} ->")) =>
                {
                    crate::i18n::fragment(s)
                        .map(|t| DiagArgValue::Str(Cow::Owned(t)))
                        .unwrap_or_else(|| value.clone())
                }
                _ => value.clone(),
            };
            (name.clone(), value)
        })
        .collect();
    let args = to_fluent_args(args.iter());
    let mut errs = vec![];
    let formatted = bundle.format_pattern(value, Some(&args), &mut errs).to_string();
    errs.is_empty().then_some(formatted)
}

fn format_fluent_str(message: &str, args: &DiagArgMap) -> Cow<'static, str> {
    trace!(?message, ?args);
    const GENERATED_MSG_ID: &str = "generated_msg";
    let resource = FluentResource::try_new(format!("{GENERATED_MSG_ID} = {message}\n")).unwrap();
    let mut bundle = fluent_bundle::FluentBundle::new(vec![langid!("en-US")]);
    bundle.set_use_isolating(false);
    bundle.add_resource(resource).unwrap();
    register_functions(&mut bundle);
    let message = bundle.get_message(GENERATED_MSG_ID).unwrap();
    let value = message.value().unwrap();
    let args = to_fluent_args(args.iter());

    let mut errs = vec![];
    let formatted = bundle.format_pattern(value, Some(&args), &mut errs).to_string();
    debug!(?formatted, ?errs);
    if errs.is_empty() {
        Cow::Owned(formatted)
    } else {
        panic!("Fluent errors while formatting message: {errs:?}");
    }
}

pub trait DiagMessageAddArg {
    fn arg(self, name: impl Into<DiagArgName>, arg: impl IntoDiagArg) -> EagerDiagMessageBuilder;
}

pub struct EagerDiagMessageBuilder {
    fluent_str: Cow<'static, str>,
    args: DiagArgMap,
}

impl DiagMessageAddArg for EagerDiagMessageBuilder {
    fn arg(
        mut self,
        name: impl Into<DiagArgName>,
        arg: impl IntoDiagArg,
    ) -> EagerDiagMessageBuilder {
        let name = name.into();
        let value = arg.into_diag_arg(&mut None);
        debug_assert!(
            !self.args.contains_key(&name) || self.args.get(&name) == Some(&value),
            "arg {} already exists",
            name
        );
        self.args.insert(name, value);
        self
    }
}

impl DiagMessageAddArg for DiagMessage {
    fn arg(self, name: impl Into<DiagArgName>, arg: impl IntoDiagArg) -> EagerDiagMessageBuilder {
        let DiagMessage::Inline(fluent_str) = self else {
            panic!("Tried to eagerly format an already formatted message")
        };
        EagerDiagMessageBuilder { fluent_str, args: Default::default() }.arg(name, arg)
    }
}

impl EagerDiagMessageBuilder {
    pub fn format(self) -> DiagMessage {
        if let Some(translated) = crate::i18n::template(&self.fluent_str)
            .and_then(|translated| try_format_fluent_str(translated, &self.args))
        {
            return DiagMessage::Str(Cow::Owned(translated));
        }
        DiagMessage::Str(format_fluent_str(&self.fluent_str, &self.args))
    }
}
