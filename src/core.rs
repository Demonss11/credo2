use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
