use crate::core::{Action, Condition, Rule, Value};
use regex::Regex;

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
