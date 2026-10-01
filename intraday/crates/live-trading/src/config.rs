//! Loads `config/tune30_live.toml` — the per-stock tuned (lookback, MRM,
//! give-back trail) parameters this is all built around. Promoted out of
//! `research/` because it's load-bearing config for real trading now, not
//! a loose research artifact.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct StockTuning {
    pub symbol: String,
    pub lookback: u32,
    pub mrm: f64,
    pub trail: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct Shared {
    risk_pct: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct RawConfig {
    shared: Shared,
    stock: Vec<StockTuning>,
}

#[derive(Debug, Clone)]
pub struct Tune30Config {
    pub risk_pct: f64,
    pub stocks: Vec<StockTuning>,
}

impl Tune30Config {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("reading {}: {e}", path.display()))?;
        let raw: RawConfig = toml::from_str(&text)
            .map_err(|e| anyhow::anyhow!("parsing {}: {e}", path.display()))?;
        if raw.stock.is_empty() {
            anyhow::bail!("{} has no [[stock]] entries", path.display());
        }
        let mut seen = std::collections::HashSet::new();
        for s in &raw.stock {
            if !seen.insert(s.symbol.clone()) {
                anyhow::bail!("{} has a duplicate [[stock]] entry for {}", path.display(), s.symbol);
            }
        }
        Ok(Self {
            risk_pct: raw.shared.risk_pct,
            stocks: raw.stock,
        })
    }

    pub fn symbols(&self) -> Vec<String> {
        self.stocks.iter().map(|s| s.symbol.clone()).collect()
    }

    pub fn by_symbol(&self) -> HashMap<String, StockTuning> {
        self.stocks.iter().map(|s| (s.symbol.clone(), s.clone())).collect()
    }
}
