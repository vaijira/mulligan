use serde::{Deserialize, Serialize};
use std::fmt;

pub(crate) const ASSETS_PATH: &str = "Assets";
pub(crate) const LIABILITIES_PATH: &str = "Liabilities";
pub(crate) const CAPITAL_PATH: &str = "Capital";

/// Use for intermediate series that contains children but no data.
pub const UNDEFINED_SERIES_NAME: &str = "UNDEFINED";

const PATH_SEPARATOR: char = '/';

/// Accounting concept.
/// A concept follows a tree structure where the leaf nodes contains
/// the final accounting values
/// and the intermediates nodes group leaves or non-leaves nodes.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Concept {
    /// accouting concept path, string separated by '/'.
    pub path: String,
    /// series where these concept was extracted from.
    pub series_name: String,
    /// accounting concept value.
    pub value: i64,
    pub(crate) children: Vec<Concept>,
}

impl Concept {
    /// concept name.
    pub fn name(&self) -> &str {
        if let Some(i) = self.path.rfind(PATH_SEPARATOR) {
            return &self.path[i + 1..];
        }
        &self.path[..]
    }

    /// Create a new concept.
    pub fn new(path: &str, series: &str) -> Concept {
        Concept {
            path: path.to_string(),
            series_name: series.to_string(),
            value: 0,
            children: Vec::new(),
        }
    }

    /// Return true if concept is a leaf concept.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Insert a concept inside the concept tree structure
    /// creating intermediate concepts if necessary.
    pub fn insert_concept(&mut self, path: &str, series: &str) {
        let mut concept = self;
        for i in path
            .char_indices()
            .filter(|c| c.1 == PATH_SEPARATOR)
            .map(|v| v.0)
        {
            if concept.path == path[..i] {
                continue;
            } else {
                let child_pos = concept
                    .children
                    .iter()
                    .position(|c| c.path == path[..i])
                    .map_or(usize::MAX, |p| p);
                if child_pos != usize::MAX {
                    concept = concept
                        .children
                        .iter_mut()
                        .find(|c| c.path == path[..i])
                        .unwrap();
                } else {
                    concept
                        .children
                        .push(Concept::new(&path[..i], UNDEFINED_SERIES_NAME));
                    concept = concept.children.last_mut().unwrap();
                }
            }
        }
        concept.children.push(Concept::new(path, series));
    }

    /// update the concept specified by path with its accounting value.
    pub fn update_concept_value(&mut self, path: &str, value: i64) {
        if path == ASSETS_PATH || path == LIABILITIES_PATH || path == CAPITAL_PATH {
            self.value = value;
            return;
        }
        let mut concept = self;
        for i in path
            .char_indices()
            .filter(|c| c.1 == PATH_SEPARATOR)
            .map(|v| v.0)
        {
            if concept.path == path[..i] {
                continue;
            } else {
                concept = concept
                    .children
                    .iter_mut()
                    .find(|c| c.path == path[..i])
                    .unwrap();
            }
        }
        concept
            .children
            .iter_mut()
            .find(|c| c.path == path)
            .unwrap()
            .value = value;
    }

    /// Provides the iterator over concepts
    pub fn iter(&self) -> crate::iter::Iter<'_> {
        crate::iter::Iter::new(self)
    }
}

impl fmt::Display for Concept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recurse_concepts(c: &Concept, i: usize, fo: &mut fmt::Formatter<'_>) -> fmt::Result {
            if !c.is_leaf()
                && !c.children.is_empty()
                && c.children
                    .iter()
                    .filter(|x| x.is_leaf() && x.value == 0)
                    .count()
                    == c.children.len()
            {
                return Ok(());
            }
            if c.series_name == UNDEFINED_SERIES_NAME && c.value == 0 {
                writeln!(fo, "{:<68.68}", " ".repeat(i * 2) + c.name())?;
            } else if !(c.is_leaf() && c.value == 0) {
                writeln!(fo, "{:<68.68}{:>12}", " ".repeat(i * 2) + c.name(), c.value)?;
            }

            for child in &c.children {
                recurse_concepts(child, i + 1, fo)?;
            }
            Ok(())
        }
        writeln!(f)?;
        recurse_concepts(self, 0, f)
    }
}

/// Type of balance sheet concepts
pub enum ConceptType {
    /// Assets in balance sheet
    Assets,
    /// Liabilities in balance sheet
    Liabilities,
    /// Capital in balance sheet
    Capital,
}

/// Balance sheet containing assets, liabilities and capital.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BalanceSheet {
    assets: Concept,
    liabilities: Concept,
    capital: Concept,
}

impl BalanceSheet {
    /// Create new balance sheet from concepts.
    pub fn new(assets: Concept, liabilities: Concept, capital: Concept) -> BalanceSheet {
        BalanceSheet {
            assets,
            liabilities,
            capital,
        }
    }

    /// Get a reference to a concrete concept.
    pub fn get_concept(&self, ctype: &ConceptType) -> &Concept {
        match ctype {
            ConceptType::Assets => &self.assets,
            ConceptType::Liabilities => &self.liabilities,
            ConceptType::Capital => &self.capital,
        }
    }

    /// Get a mutable reference from a concrete concept.
    pub fn get_concept_mut(&mut self, ctype: &ConceptType) -> &mut Concept {
        match ctype {
            ConceptType::Assets => &mut self.assets,
            ConceptType::Liabilities => &mut self.liabilities,
            ConceptType::Capital => &mut self.capital,
        }
    }
}

impl fmt::Display for BalanceSheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Balance sheet\n{}\n{}\n{}\n",
            self.assets, self.liabilities, self.capital
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fed;

    #[test]
    fn name_test() {
        let mut assets = Concept::new(ASSETS_PATH, fed::FED_ASSETS_SERIES_NAME);
        assert_eq!("Assets", assets.name());
        assets.insert_concept("Assets/Liquidity and Credit Facilities/Net portfolio holdings of Commercial Paper Funding Facility LLC", UNDEFINED_SERIES_NAME);
        assert_eq!(
            "Liquidity and Credit Facilities",
            assets.children.last().unwrap().name()
        );
        assert_eq!(
            "Net portfolio holdings of Commercial Paper Funding Facility LLC",
            assets
                .children
                .last()
                .unwrap()
                .children
                .last()
                .unwrap()
                .name()
        );
    }

    #[test]
    fn insert_concept_test() {
        let mut assets = Concept::new(ASSETS_PATH, fed::FED_ASSETS_SERIES_NAME);
        assets.insert_concept("Assets/Liquidity and Credit Facilities/Net portfolio holdings of Commercial Paper Funding Facility LLC", UNDEFINED_SERIES_NAME);
        assert_eq!(assets.path, "Assets");
        assert_eq!(
            assets.children.last().unwrap().path,
            "Assets/Liquidity and Credit Facilities"
        );
        assert_eq!(assets.children.last().unwrap().children.last().unwrap().path, "Assets/Liquidity and Credit Facilities/Net portfolio holdings of Commercial Paper Funding Facility LLC");
    }

    #[test]
    fn update_concept_value_test() {
        let mut assets = Concept::new(ASSETS_PATH, fed::FED_ASSETS_SERIES_NAME);
        assets.insert_concept("Assets/Liquidity and Credit Facilities/Net portfolio holdings of Commercial Paper Funding Facility LLC", UNDEFINED_SERIES_NAME);
        assets.update_concept_value("Assets/Liquidity and Credit Facilities", 4);
        assert_eq!(assets.children.last().unwrap().value, 4);
        assets.update_concept_value("Assets/Liquidity and Credit Facilities/Net portfolio holdings of Commercial Paper Funding Facility LLC", 8);
        assert_eq!(
            assets
                .children
                .last()
                .unwrap()
                .children
                .last()
                .unwrap()
                .value,
            8
        );
    }
}
