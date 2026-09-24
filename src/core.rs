use anyhow::{Result, bail};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

// ---------- доменные типы ----------

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    Str(String),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Str(_) => "string",
        }
    }
    pub fn display(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Str(s) => format!("{s:?}"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub op: String,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub decision: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub condition: Condition,
    pub action: Action,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckContract {
    pub name: String,
    pub version: String,
    pub description: String,
    pub inputs: Vec<FieldSchema>,
    pub decisions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckMeta {
    pub name: String,
    pub version: String,
    pub published_at: String,
    pub published_by: String,
    pub checksum: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
}

#[derive(Clone, Debug)]
pub struct StoredCheck {
    pub name: String,
    pub version: String,
    pub rule: Rule,
    pub contract: CheckContract,
    pub meta: CheckMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Explanation {
    pub rule_name: String,
    pub decision: String,
    pub reason: String,
    pub actual_value: Value,
    pub matched: bool,
}

pub fn evaluate_rule(rule: &Rule, input: &HashMap<String, Value>) -> Explanation {
    let actual = input
        .get(&rule.condition.field)
        .cloned()
        .unwrap_or(Value::Number(0.0));
    let matched = match (&actual, rule.condition.op.as_str(), &rule.condition.value) {
        (Value::Number(a), "<", Value::Number(b)) => a < b,
        (Value::Number(a), ">", Value::Number(b)) => a > b,
        (Value::Number(a), "==", Value::Number(b)) => a == b,
        (Value::Number(a), "!=", Value::Number(b)) => a != b,
        (Value::Str(a), "==", Value::Str(b)) => a == b,
        (Value::Str(a), "!=", Value::Str(b)) => a != b,
        _ => false,
    };
    Explanation {
        rule_name: rule.name.clone(),
        decision: if matched {
            rule.action.decision.clone()
        } else {
            String::new()
        },
        reason: if matched {
            rule.action.reason.clone()
        } else {
            String::new()
        },
        actual_value: actual,
        matched,
    }
}

pub fn contract_from_rule(rule: &Rule, version: &str) -> CheckContract {
    CheckContract {
        name: rule.name.clone(),
        version: version.to_string(),
        description: format!(
            "Если {} {} {} → {}",
            rule.condition.field,
            rule.condition.op,
            rule.condition.value.display(),
            rule.action.decision,
        ),
        inputs: vec![FieldSchema {
            name: rule.condition.field.clone(),
            type_: rule.condition.value.type_name().into(),
            description: None,
        }],
        decisions: vec![rule.action.decision.clone(), "Pass".into()],
    }
}

/// Сравнение контрактов только по входам — решает, нужен ли MAJOR bump.
pub fn same_inputs(a: &CheckContract, b: &CheckContract) -> bool {
    if a.inputs.len() != b.inputs.len() {
        return false;
    }
    let mut x: Vec<_> = a.inputs.iter().map(|f| (&f.name, &f.type_)).collect();
    let mut y: Vec<_> = b.inputs.iter().map(|f| (&f.name, &f.type_)).collect();
    x.sort();
    y.sort();
    x == y
}

pub fn checksum_of(rule: &Rule, contract: &CheckContract) -> anyhow::Result<String> {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(serde_json::to_vec(rule)?);
    h.update(b"|");
    h.update(serde_json::to_vec(contract)?);
    Ok(format!("sha256:{}", hex::encode(h.finalize())))
}

// ---------- semver ----------

#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
pub struct Semver {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
    pub build: Option<String>,
}

// `PartialEq` должен быть согласован с `Ord` (контракт `Ord`:
// `a == b ⟺ a.cmp(b) == Equal`), поэтому build-метаданные игнорируются.
impl PartialEq for Semver {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
            && self.pre == other.pre
    }
}

impl Semver {
    pub fn parse(input: &str) -> Result<Self> {
        let s = input.strip_prefix('v').unwrap_or(input);
        if s.is_empty() {
            bail!("пустая версия");
        }
        if s.starts_with('.') {
            bail!("версия не может начинаться с точки: {input:?}");
        }

        let (s, build) = match s.split_once('+') {
            Some((a, b)) => {
                if b.is_empty() {
                    bail!("пустой build-метаданные");
                }
                if !b
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'.' || c == b'-')
                {
                    bail!("невалидные build-метаданные");
                }
                if b.split('.').any(|seg| seg.is_empty()) {
                    bail!("пустой идентификатор в build-метаданных");
                }
                (a, Some(b.to_string()))
            }
            None => (s, None),
        };

        let (s, pre) = match s.split_once('-') {
            Some((a, b)) => {
                if b.is_empty() {
                    bail!("пустой pre-release");
                }
                if !b
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'.' || c == b'-')
                {
                    bail!("невалидный pre-release");
                }
                if b.split('.').any(|seg| seg.is_empty()) {
                    bail!("пустой идентификатор в pre-release");
                }
                (a, Some(b.to_string()))
            }
            None => (s, None),
        };

        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            bail!("нужны ровно три компонента MAJOR.MINOR.PATCH");
        }
        let major = parse_num(parts[0])?;
        let minor = parse_num(parts[1])?;
        let patch = parse_num(parts[2])?;
        Ok(Self {
            major,
            minor,
            patch,
            pre,
            build,
        })
    }

    /// Строка версии для хранилища/путей: без `+build` (build не влияет на
    /// идентичность версии и не должен попадать в путь `checks/{name}/{version}`).
    pub fn as_storage(&self) -> String {
        let base = format!("{}.{}.{}", self.major, self.minor, self.patch);
        match &self.pre {
            Some(p) => format!("{base}-{p}"),
            None => base,
        }
    }
}

impl fmt::Display for Semver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(p) = &self.pre {
            write!(f, "-{p}")?;
        }
        if let Some(b) = &self.build {
            write!(f, "+{b}")?;
        }
        Ok(())
    }
}

fn parse_num(s: &str) -> Result<u64> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        bail!("компонент не число: {s:?}");
    }
    if s.len() > 1 && s.starts_with('0') {
        bail!("leading zero в компоненте версии: {s:?}");
    }
    s.parse()
        .map_err(|_| anyhow::anyhow!("компонент слишком велик"))
}

/// Сравнение pre-release по правилам semver: сегменты слева направо,
/// числовые идентификаторы — численно, числовой < буквенно-цифрового,
/// буквенно-цифровые — лексикографически; большее число сегментов выше.
fn compare_pre(a: &str, b: &str) -> Ordering {
    let mut ai = a.split('.');
    let mut bi = b.split('.');
    loop {
        match (ai.next(), bi.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) => {
                let ord = compare_ident(x, y);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
        }
    }
}

fn compare_ident(a: &str, b: &str) -> Ordering {
    let a_num = a.bytes().all(|c| c.is_ascii_digit());
    let b_num = b.bytes().all(|c| c.is_ascii_digit());
    match (a_num, b_num) {
        (true, true) => {
            let av: u64 = a.parse().unwrap_or(u64::MAX);
            let bv: u64 = b.parse().unwrap_or(u64::MAX);
            av.cmp(&bv)
        }
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => a.cmp(b),
    }
}

// Semver-приоритет: build игнорируется, pre сортируется ниже release.
impl Ord for Semver {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch)
            .cmp(&(other.major, other.minor, other.patch))
            .then_with(|| match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => compare_pre(a, b),
            })
    }
}

impl PartialOrd for Semver {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

// ---------- DSL ----------

pub fn parse_rule(source: &str) -> Result<Rule, String> {
    let name_re = Regex::new(r"Правило\s+(\w+)").unwrap();
    let cond_re = Regex::new(r"Если\s*\(([^<>=!]+)\s*(<|>|==|!=)\s*([^)]+)\)").unwrap();
    let decision_re = Regex::new(r"Решение\s*=\s*(\w+)").unwrap();
    let reason_re = Regex::new(r#"Причина\s*=\s*"([^"]+)""#).unwrap();

    let name = name_re
        .captures(source)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or("Не найдено имя правила")?;
    let cond = cond_re.captures(source).ok_or("Не найдено условие")?;
    let field = cond.get(1).unwrap().as_str().trim().to_string();
    let op = cond.get(2).unwrap().as_str().to_string();
    let value_str = cond.get(3).unwrap().as_str().trim();

    let value = if let Ok(n) = value_str.parse::<f64>() {
        Value::Number(n)
    } else {
        Value::Str(value_str.trim_matches('"').to_string())
    };

    let decision = decision_re
        .captures(source)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or("Не найдено решение")?;
    let reason = reason_re
        .captures(source)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_default();

    Ok(Rule {
        name,
        condition: Condition { field, op, value },
        action: Action { decision, reason },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(s: &str) -> String {
        Semver::parse(s).unwrap().to_string()
    }

    #[test]
    fn parse_accepts_v_prefix() {
        assert_eq!(ok("v1.2.3"), "1.2.3");
    }
    #[test]
    fn parse_rejects_v_dot() {
        assert!(Semver::parse("v.0.0.1").is_err());
    }
    #[test]
    fn parse_rejects_two_components() {
        assert!(Semver::parse("1.2").is_err());
    }
    #[test]
    fn parse_rejects_four_components() {
        assert!(Semver::parse("1.2.3.4").is_err());
    }
    #[test]
    fn parse_rejects_x() {
        assert!(Semver::parse("1.x.3").is_err());
    }
    #[test]
    fn parse_rejects_empty() {
        assert!(Semver::parse("").is_err());
    }
    #[test]
    fn prerelease_sorts_below_release() {
        let rc = Semver::parse("1.0.0-rc1").unwrap();
        let rel = Semver::parse("1.0.0").unwrap();
        assert!(rc < rel);
    }
    #[test]
    fn major_priority() {
        let a = Semver::parse("2.0.0").unwrap();
        let b = Semver::parse("1.99.99").unwrap();
        assert!(a > b);
    }
    #[test]
    fn compare_semver_returns_error() {
        // сравнение — не парсинг; ошибка возвращается парсером
        assert!(Semver::parse("abc").is_err());
    }
    #[test]
    fn build_metadata_preserved() {
        assert_eq!(ok("1.2.3+build"), "1.2.3+build");
    }
    #[test]
    fn eq_ignores_build_metadata_consistent_with_ord() {
        let a = Semver::parse("1.2.3+a").unwrap();
        let b = Semver::parse("1.2.3+b").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }
    #[test]
    fn prerelease_numeric_ordering() {
        // числовые идентификаторы сравниваются численно: 2 < 10
        let a = Semver::parse("1.0.0-alpha.2").unwrap();
        let b = Semver::parse("1.0.0-alpha.10").unwrap();
        assert!(a < b);
    }
    #[test]
    fn numeric_identifier_below_alphanumeric() {
        let num = Semver::parse("1.0.0-1").unwrap();
        let alpha = Semver::parse("1.0.0-alpha").unwrap();
        assert!(num < alpha);
    }
    #[test]
    fn more_prerelease_fields_have_higher_precedence() {
        let a = Semver::parse("1.0.0-alpha").unwrap();
        let b = Semver::parse("1.0.0-alpha.1").unwrap();
        assert!(a < b);
    }
    #[test]
    fn rejects_leading_dot_in_pre() {
        assert!(Semver::parse("1.0.0-.rc").is_err());
    }
    #[test]
    fn rejects_trailing_dot_in_pre() {
        assert!(Semver::parse("1.0.0-rc.").is_err());
    }
    #[test]
    fn rejects_empty_segment_in_pre() {
        assert!(Semver::parse("1.0.0-rc..1").is_err());
    }
    #[test]
    fn rejects_empty_segment_in_build() {
        assert!(Semver::parse("1.0.0+build..1").is_err());
    }
    #[test]
    fn rejects_leading_zero() {
        assert!(Semver::parse("01.2.3").is_err());
        assert!(Semver::parse("1.02.3").is_err());
        assert!(Semver::parse("1.2.03").is_err());
    }
    #[test]
    fn as_storage_drops_build() {
        assert_eq!(
            Semver::parse("1.2.3+build.7").unwrap().as_storage(),
            "1.2.3"
        );
        assert_eq!(
            Semver::parse("1.2.3-rc.1+build").unwrap().as_storage(),
            "1.2.3-rc.1"
        );
    }
}
