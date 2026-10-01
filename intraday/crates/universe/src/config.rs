use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct UniverseConfig {
    pub seed: String,
    pub min_avg_turnover_cr: f64,
    pub min_atr_pct: f64,
    pub min_avg_volume: f64,
    pub lookback_days: u32,
    pub max_symbols: usize,
    /// Symbols barred from every universe regardless of liquidity or cached
    /// coverage — for instruments whose *upstream* data is known bad, which
    /// no screen here can detect. Kept as config rather than a filter at each
    /// call site so a bad instrument can't leak back in through one query
    /// someone forgot to guard.
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl UniverseConfig {
    pub fn seed_index(&self) -> anyhow::Result<crate::seed::SeedIndex> {
        crate::seed::SeedIndex::parse(&self.seed)
    }

    pub fn is_excluded(&self, tradingsymbol: &str) -> bool {
        self.exclude
            .iter()
            .any(|s| s.eq_ignore_ascii_case(tradingsymbol))
    }
}

#[derive(Debug, Deserialize)]
struct RootConfig {
    universe: UniverseConfig,
}

pub fn load(path: &Path) -> anyhow::Result<UniverseConfig> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("reading {path:?}: {e}"))?;
    let root: RootConfig = toml::from_str(&text)
        .map_err(|e| anyhow::anyhow!("parsing [universe] section of {path:?}: {e}"))?;
    Ok(root.universe)
}

pub fn load_default() -> anyhow::Result<UniverseConfig> {
    load(Path::new("config/default.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(exclude: &[&str]) -> UniverseConfig {
        UniverseConfig {
            seed: "nifty250".into(),
            min_avg_turnover_cr: 50.0,
            min_atr_pct: 0.8,
            min_avg_volume: 500_000.0,
            lookback_days: 20,
            max_symbols: 20,
            exclude: exclude.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn excludes_listed_symbol_and_leaves_others_alone() {
        let c = cfg(&["PIIND"]);
        assert!(c.is_excluded("PIIND"));
        assert!(!c.is_excluded("VOLTAS"));
    }

    #[test]
    fn exclusion_is_case_insensitive_so_a_lowercase_config_entry_still_bites() {
        assert!(cfg(&["piind"]).is_excluded("PIIND"));
        assert!(cfg(&["PIIND"]).is_excluded("piind"));
    }

    #[test]
    fn empty_exclude_list_excludes_nothing() {
        assert!(!cfg(&[]).is_excluded("PIIND"));
    }

    #[test]
    fn exclude_defaults_to_empty_when_absent_from_the_toml() {
        // The field is #[serde(default)] — an older config without the key
        // must still parse rather than failing every command that loads it.
        let toml = r#"
            [universe]
            seed = "nifty100"
            min_avg_turnover_cr = 50.0
            min_atr_pct = 0.8
            min_avg_volume = 500000
            lookback_days = 20
            max_symbols = 20
        "#;
        let root: RootConfig = toml::from_str(toml).expect("parses without exclude");
        assert!(root.universe.exclude.is_empty());
    }

    #[test]
    fn the_shipped_default_config_actually_excludes_piind() {
        // Guards the real thing: if someone edits config/default.toml and drops
        // the entry, PIIND's corrupt upstream data silently re-enters research.
        // Resolved from CARGO_MANIFEST_DIR rather than the CWD so this asserts
        // against the real file wherever the test is invoked from — a relative
        // path that silently fails to load would make this test vacuous.
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/default.toml");
        let c = load(&path).expect("the shipped config/default.toml must load");
        assert!(
            c.is_excluded("PIIND"),
            "config/default.toml must keep PIIND excluded — Kite serves \
             cross-contaminated bars for it and re-fetching cannot fix it"
        );
    }
}
