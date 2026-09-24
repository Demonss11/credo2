use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Semver {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
    pub build: Option<String>,
}

impl Semver {
    pub fn parse(input: &str) -> Result<Self> {
        let s = input.strip_prefix('v').unwrap_or(input);
        if s.is_empty() {
            bail!("пустая версия");
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
    s.parse()
        .map_err(|_| anyhow::anyhow!("компонент слишком велик"))
}

// Semver-приоритет: build игнорируется, pre сортируется ниже release,
// лексикографически (TODO: numeric pre-release, см. отложенные требования).
impl Ord for Semver {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch)
            .cmp(&(other.major, other.minor, other.patch))
            .then_with(|| match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => a.cmp(b),
            })
    }
}

impl PartialOrd for Semver {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
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
}
