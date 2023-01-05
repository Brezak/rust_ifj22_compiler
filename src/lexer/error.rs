use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::error::Simple;

use crate::extensions::ToColor;

pub fn print_lexer_errors(errs: Vec<Simple<char>>, src: &str, src_name: &str) {
    for err in errs {
        let kind;
        let msg = if let chumsky::error::SimpleReason::Custom(msg) = err.reason() {
            if let Some((reason, message)) = msg.split_once(':') {
                if reason == "Warn" {
                    kind = ReportKind::Warning;
                } else {
                    kind = ReportKind::Error;
                }

                message.to_string()
            } else {
                kind = ReportKind::Error;
                msg.to_string()
            }
        } else {
            kind = ReportKind::Error;
            format!(
                "{}{}, expected {}",
                if err.found().is_some() {
                    "Unexpected token"
                } else {
                    "Unexpected end of input"
                },
                if let Some(label) = err.label() {
                    format!(" while parsing {}", label)
                } else {
                    String::new()
                },
                if err.expected().len() == 0 {
                    "something else".to_string()
                } else {
                    err.expected()
                        .map(|expected| match expected {
                            Some(expected) => expected.to_string(),
                            None => "end of input".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                },
            )
        };

        let report = Report::build(kind, src_name, err.span().start)
            .with_code(3)
            .with_message(&msg)
            .with_label(
                Label::new((src_name, err.span()))
                    .with_message(match err.reason() {
                        chumsky::error::SimpleReason::Custom(_) => msg.clone(),
                        _ => format!(
                            "Unexpected {}",
                            err.found()
                                .map(|c| format!("token {}", c.fg(kind.color())))
                                .unwrap_or_else(|| "end of input".to_string())
                        ),
                    })
                    .with_color(kind.color()),
            );

        let report = match err.reason() {
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report.with_label(
                Label::new((src_name, span.clone()))
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_color(Color::Yellow),
            ),
            chumsky::error::SimpleReason::Unexpected => report,
            chumsky::error::SimpleReason::Custom(_) => report,
        };

        report
            .finish()
            .print((src_name, Source::from(&src)))
            .unwrap();
    }
}
