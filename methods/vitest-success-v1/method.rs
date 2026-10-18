// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Donald Tong
// Original AI-assisted parser and synthetic tests; this does not claim every
// Vitest 4.0.0 environment has been verified.
//! Strict parser for one Vitest 4.0.0 default success summary.
//! Unknown output stays on the caller's existing path.

/// Return a compact summary only for the complete, all-passing Vitest 4.0.0
/// default multi-file report emitted by `vitest run` or `npx vitest run`.
pub fn summarize_vitest_success(command: &str, text: &str, exit_status: i32) -> Option<String> {
    if exit_status != 0
        || command
            .chars()
            .any(|character| matches!(character, '&' | '|' | ';' | '>' | '<' | '`' | '\r' | '\n'))
        || command.contains("$(")
        || !matches!(
            command.split_whitespace().collect::<Vec<_>>().as_slice(),
            ["vitest", "run"] | ["npx", "vitest", "run"]
        )
        || text.contains('\u{1b}')
        || text
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\r'))
        || text
            .as_bytes()
            .iter()
            .enumerate()
            .any(|(index, byte)| *byte == b'\r' && text.as_bytes().get(index + 1) != Some(&b'\n'))
    {
        return None;
    }

    let lines: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    let mut index = 0;
    let mut header = lines.get(index)?.split_whitespace();
    if header.next()? != "RUN" || header.next()? != "v4.0.0" || header.next().is_none() {
        return None;
    }
    if header.next().is_some() {
        return None;
    }
    index += 1;

    let mut test_files = 0usize;
    let mut tests = 0usize;
    while let Some(line) = lines.get(index).filter(|line| line.starts_with("✓ ")) {
        let content = line.strip_prefix("✓ ")?;
        let (file, summary) = content.rsplit_once(" (")?;
        if !(file.contains(".test.") || file.contains(".spec.")) {
            return None;
        }
        let (count, duration) = summary.split_once(") ")?;
        let (count, unit) = count.split_once(' ')?;
        let count = count.parse::<usize>().ok()?;
        if count == 0
            || (count == 1 && unit != "test")
            || (count > 1 && unit != "tests")
            || !is_ms(duration)
        {
            return None;
        }
        test_files += 1;
        tests = tests.checked_add(count)?;
        index += 1;
    }
    if test_files < 2 || tests == 0 {
        return None;
    }

    let (passed_files, total_files) = parse_counts(lines.get(index)?, "Test Files")?;
    if passed_files != test_files || total_files != test_files {
        return None;
    }
    index += 1;
    let (passed_tests, total_tests) = parse_counts(lines.get(index)?, "Tests")?;
    if passed_tests != tests || total_tests != tests {
        return None;
    }
    index += 1;

    let start_at = lines.get(index)?.strip_prefix("Start at ")?.trim();
    if start_at.is_empty() || !start_at.contains(':') {
        return None;
    }
    index += 1;

    let duration = lines.get(index)?.strip_prefix("Duration ")?;
    let (elapsed, phases) = duration.split_once(" (")?;
    if !is_ms(elapsed) {
        return None;
    }
    let expected_phases = [
        "transform",
        "setup",
        "collect",
        "tests",
        "environment",
        "prepare",
    ];
    let actual_phases: Vec<&str> = phases
        .strip_suffix(')')?
        .split(", ")
        .map(|phase| {
            let (name, value) = phase.split_once(' ')?;
            is_ms(value).then_some(name)
        })
        .collect::<Option<Vec<_>>>()?;
    if actual_phases.as_slice() != expected_phases.as_slice() || index + 1 != lines.len() {
        return None;
    }

    Some(format!(
        "Vitest v4.0.0: {passed_files}/{total_files} test files passed; {passed_tests}/{total_tests} tests passed\nStart at {start_at}\nDuration {elapsed}"
    ))
}

fn parse_counts(line: &str, label: &str) -> Option<(usize, usize)> {
    let values: Vec<&str> = line.strip_prefix(label)?.split_whitespace().collect();
    if values.len() != 3 || values[1] != "passed" {
        return None;
    }
    let passed = values[0].parse::<usize>().ok()?;
    let total = values[2]
        .strip_prefix('(')?
        .strip_suffix(')')?
        .parse::<usize>()
        .ok()?;
    Some((passed, total))
}

fn is_ms(value: &str) -> bool {
    let Some(number) = value.strip_suffix("ms") else {
        return false;
    };
    let mut parts = number.split('.');
    let Some(whole) = parts.next() else {
        return false;
    };
    if whole.is_empty() || !whole.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }
    match parts.next() {
        None => true,
        Some(fraction) => {
            !fraction.is_empty()
                && fraction.bytes().all(|byte| byte.is_ascii_digit())
                && parts.next().is_none()
        }
    }
}

#[cfg(test)]
pub(crate) const COMPLETE_VITEST_SUCCESS_REPORT: &str = " RUN  v4.0.0 C:/project\n\
 ✓ src/alpha.test.ts (2 tests) 4ms\n\
 ✓ src/beta.test.ts (3 tests) 5ms\n\
\n\
 Test Files  2 passed (2)\n\
      Tests  5 passed (5)\n\
   Start at  13:29:16\n\
   Duration 24ms (transform 8ms, setup 0ms, collect 6ms, tests 4ms, environment 0ms, prepare 4ms)\n";

#[cfg(test)]
mod tests {
    use super::{COMPLETE_VITEST_SUCCESS_REPORT, summarize_vitest_success};

    fn replace_fixture_text(from: &str, to: &str) -> String {
        assert!(COMPLETE_VITEST_SUCCESS_REPORT.contains(from));
        let changed = COMPLETE_VITEST_SUCCESS_REPORT.replacen(from, to, 1);
        assert_ne!(changed, COMPLETE_VITEST_SUCCESS_REPORT);
        changed
    }

    #[test]
    fn vitest_success_v1_accepts_only_the_verified_complete_all_pass_shape() {
        let summary = summarize_vitest_success("vitest run", COMPLETE_VITEST_SUCCESS_REPORT, 0)
            .expect("verified report should compact");
        assert!(summary.contains("Vitest v4.0.0"));
        assert!(summary.contains("2/2 test files passed"));
        assert!(summary.contains("5/5 tests passed"));
        assert!(summary.contains("Duration 24ms"));
        assert!(
            summarize_vitest_success("npx vitest run", COMPLETE_VITEST_SUCCESS_REPORT, 0).is_some()
        );
    }

    #[test]
    fn vitest_nonzero_unknown_warning_skip_or_malformed_report_is_not_compacted() {
        let rejected = [
            (COMPLETE_VITEST_SUCCESS_REPORT.to_owned(), 1),
            (replace_fixture_text("v4.0.0", "v4.1.0"), 0),
            (
                replace_fixture_text("Tests  5 passed (5)", "Tests  4 passed | 1 skipped (5)"),
                0,
            ),
            (
                replace_fixture_text("Tests  5 passed (5)", "Tests  4 passed (4)"),
                0,
            ),
            (
                replace_fixture_text(
                    "Tests  5 passed (5)",
                    "Tests  5 passed (5)\n Unknown report line",
                ),
                0,
            ),
            (
                replace_fixture_text(
                    "Tests  5 passed (5)",
                    "Tests  5 passed (5)\n Warning: reporter warning",
                ),
                0,
            ),
            (
                {
                    let changed = COMPLETE_VITEST_SUCCESS_REPORT.replace('\n', "\r");
                    assert_ne!(changed, COMPLETE_VITEST_SUCCESS_REPORT);
                    changed
                },
                0,
            ),
        ];
        for (report, exit_status) in rejected {
            assert!(summarize_vitest_success("vitest run", &report, exit_status).is_none());
        }
        assert!(
            summarize_vitest_success("vitest run && echo done", COMPLETE_VITEST_SUCCESS_REPORT, 0)
                .is_none()
        );
    }
}
