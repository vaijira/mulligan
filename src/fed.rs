use chrono::NaiveDate;
use radix_trie::{Trie, TrieCommon};
use roxmltree::{Document, Node};
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::types;
use crate::types::{BalanceSheet, Concept};

/*
    Urls for FED H.4.1 statistical data
    https://www.federalreserve.gov/datadownload/
    https://www.federalreserve.gov/datadownload/Choose.aspx?rel=H41
    https://www.federalreserve.gov/releases/H41/20200507/
    https://www.federalreserve.gov/releases/H41/20200507/H41.TXT
*/
/// URL to download zip file containing FED H.4.1 statistical information.
pub const H41_FED_URL: &str =
    "https://www.federalreserve.gov/datadownload/Output.aspx?rel=H41&filetype=zip";

/// File name that contains the XML structure of H.4.1 data.
pub const H41_STRUCT_XML: &str = "H41_struct.xml";

/// File name containing the XML H.4.1 data.
pub const H41_DATA_XML: &str = "H41_data.xml";

/// Parent series for FED H.4.1 assets
pub(crate) const FED_ASSETS_SERIES_NAME: &str = "RESPPA_N.WW";

const STRUCTURE_NS: &str = "http://www.SDMX.org/resources/SDMXML/schemas/v1_0/structure";
const KF_NS: &str = "http://www.federalreserve.gov/structure/compact/H41_H41";
const FRB_NS: &str = "http://www.federalreserve.gov/structure/compact/common";
const COMMON_NS: &str = "http://www.SDMX.org/resources/SDMXML/schemas/v1_0/common";

/*
    FED ASSETS Series discarded for assets in balance sheet

    We have a series for gold and another for special drawing rights (duplicated)
    RESPPAR_N.WW -> subcategory(ZZZZ): Other -> component(GCA): Gold certificate account and SDR account

    This is a weekley average we are interested in Wednesday level conflict with "RESPPALGAM_N.WW"
    RESPPALGAO_N.WW -> subcategory(ORH): Securities Held Outright -> component(FADS): Federal agency debt securities

    The following series seems a duplicate of RESPPALGUO_N series
    RESPPALGUM_N.WW -> subcategory(ORH): Securities Held Outright -> component(USTS): U.S. Treasury securities

    The following series seems a duplicate of RESPPALD_N.WW series
    RESPPALDV_N.WW -> subcategory(LCF) -> component(LNC)

    It seems duplicate of "RESPPAG_N.WW"
    RESTBMG_N.WW -> subcategory(OFSRB): Other Factors Supplying Reserve Balances -> component(GS): Gold stock

    Redundant (it could be included if we preparse data so concepts belong to this component, like all securities held outright)
    RESPPAL_N.WW -> subcategory(ZZZZ): Other: -> component (SRPTACOL) -> Securities, premiums, discounts, repurchase agreements, and loans: Wednesday level

    We will include RESPPALGASMO_N.WW instead the following 2 subconcepts of mortgage back securities
    RESPPALGASMR_N.WW -> subcategory(ORH) : Securities Held Outright: component(RMSSHOR) -> Residential mortgage-backed securities
    RESPPALGASMS_N.WW -> subcategory(ORH) : Securities Held Outright: component(CMSSHOR) -> Commercial mortgage-backed securities

    We will include RESPPALGTR_N.WW instead of the following 2 subconcepts of REPOs
    RESPPALGTRO_N.WW -> subcategory(ZZZZ) : Other: component(RPF) ->  Repurchase agreements - Foreign official
    RESPPALGTRF_N.WW -> subcategory(ZZZZ) : Ohter: componennt(RPD) ->  Repurchase agreements - Others

    We will include RESPPAAC2H_N.WW (net portfolio holdins of commercial paper funcing facility II LLC) instead of their subconcepts
    RESPPAAC2MC_N.WW
    RESPPAAC2MCD15_N.WW
    RESPPAAC2MCD16T90_N.WW
    RESPPAAC2MCY01_N.WW

    No idea yet what this included but seems related with RESPPAO_N.WW + something
    RESPPAE_N.WW -> subcategory(ZZZZ) Other: component(OARS) -> Other Assets, Reserve Bank Table (post 2020-03-14)

    Redundant (this is a grouping category of notes and bonds)
    RESPPALGUON_N.WW -> subcategory(ORH) Securities Held Outright: U.s treasury securities -> component(USTSNB) Notes and bonds

    The following don't seem to fit in the balance sheet part:
    RESH4S_N.WW -> subcategory(OFSRB): Other Factors Supplying Reserve Balances -> component(FSRF): Total factors supplying reserves funds
    RESH4SC_N.WW -> subcategory(OFSRB): Other Factors Supplying Reserve Balances -> component(RBC): Reserve Bank credit
    RESH4SCF_N.WW -> subcategory(OFSRB): Other Factors Supplying Reserve Balances -> component(FLT): Float
    RESH4SO_N.WW -> subcategory(OFSRB): Other Factors Supplying Reserve Balances -> component(OFRA): Other Federal Reserve assets
    RESTBMT_N.WW -> subcategory(OFSRB): Other Factors Supplying Reserve Balances -> component(TCO): Treasury currency outstanding
*/
lazy_static! {
    static ref SERIES_TO_FILTER_OUT: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("RESPPAR_N.WW");
        m.insert("RESPPAL_N.WW");
        m.insert("RESPPAE_N.WW");
        m.insert("RESPPALGAO_N.WW");
        m.insert("RESPPALGASMR_N.WW");
        m.insert("RESPPALGASMS_N.WW");
        m.insert("RESPPALGTRO_N.WW");
        m.insert("RESPPALGTRF_N.WW");
        m.insert("RESPPAAC2MC_N.WW");
        m.insert("RESPPAAC2MCD15_N.WW");
        m.insert("RESPPAAC2MCD16T90_N.WW");
        m.insert("RESPPAAC2MCY01_N.WW");
        m.insert("RESPPALGUON_N.WW");
        m.insert("RESPPALGUM_N.WW");
        m.insert("RESPPALDV_N.WW");
        m.insert("RESTBMG_N.WW");
        m.insert("RESH4S_N.WW");
        m.insert("RESH4SC_N.WW");
        m.insert("RESH4SCF_N.WW");
        m.insert("RESH4SO_N.WW");
        m.insert("RESTBMT_N.WW");
        m
    };
}

const CODE_LIST_TAG: &str = "CodeList";
const CODE_TAG: &str = "Code";
const SERIES_TAG: &str = "Series";
const OBS_TAG: &str = "Obs";
const ANNOTATION_TEXT_TAG: &str = "AnnotationText";

/// Ordered map containing balance sheet grouped for each date.
pub type ObservationMap = BTreeMap<NaiveDate, BalanceSheet>;

type ConceptMap = HashMap<String, HashMap<String, String>>;

fn get_asset_series<'a>(doc: &'a Document<'_>) -> Vec<Node<'a, 'a>> {
    doc.descendants()
        .filter(|n| {
            n.is_element()
                && n.has_tag_name((KF_NS, SERIES_TAG))
                && n.attribute("CATEGORY") == Some("ASSET")
                && n.attribute("DISTRIBUTION") == Some("TOT")
                && n.attribute("SERIESTYPE") == Some("L")
                && n.attribute("FREQ") == Some("19")
                && !SERIES_TO_FILTER_OUT.contains(n.attribute("SERIES_NAME").unwrap())
        })
        .collect()
}

fn get_node_elements<'a>(serie: &'a Node<'_, '_>, ns: &str, tag: &str) -> Vec<Node<'a, 'a>> {
    serie
        .descendants()
        .filter(|n| n.is_element() && n.has_tag_name((ns, tag)))
        .collect()
}

fn get_children_node_elements<'a>(
    serie: &'a Node<'_, '_>,
    ns: &str,
    tag: &str,
) -> Vec<Node<'a, 'a>> {
    serie
        .children()
        .filter(|n| n.is_element() && n.has_tag_name((ns, tag)))
        .collect()
}

fn get_annotation(serie: &Node<'_, '_>) -> String {
    let annotation_texts = get_node_elements(serie, COMMON_NS, ANNOTATION_TEXT_TAG);

    annotation_texts
        .first()
        .unwrap()
        .text()
        .unwrap()
        .to_string()
}

fn parse_annotation(annotation: &str) -> String {
    let path = annotation.replace(": Wednesday level", "");
    let path = path.replace(": Securities Held Outright: Securities held outright", ": Securities Held Outright");
    let path = path.replace(": All", "");
    let path = path.replace("Discontinued: ", "");
    let path = path.replace(": ", "/");
    let path = path.replace("Assets /", "Assets/");
    if path == "Assets/Total Assets/Total assets" {
        return types::ASSETS_PATH.to_string();
    }
    if !path.starts_with("Assets/") {
        "Assets/".to_string() + &path
    } else {
        path
    }
}

fn paths_to_balance_sheet_assets(paths: Trie<String, String>) -> BalanceSheet {
    let mut assets = Concept::new(types::ASSETS_PATH, FED_ASSETS_SERIES_NAME);

    for (path, series_name) in paths.iter() {
        if path == types::ASSETS_PATH {
            continue;
        }
        assets.insert_concept(path, series_name);
    }

    BalanceSheet { assets }
}

#[allow(dead_code)]
fn parse_h41_struct(text: &String) -> Result<ConceptMap, Box<dyn std::error::Error>> {
    let doc = Document::parse(&text)?;
    let root = doc.root();
    let mut concepts: ConceptMap = HashMap::new();
    let mut codelists = get_node_elements(&root, STRUCTURE_NS, CODE_LIST_TAG);

    for codelist in &mut codelists {
        let id = codelist
            .attribute("id")
            .expect("CodeList XML node should have an id attribute");
        concepts.insert(id.to_string(), HashMap::new());
        let mut codes = get_children_node_elements(&codelist, STRUCTURE_NS, CODE_TAG);

        for code in &mut codes {
            let key = code
                .attribute("value")
                .expect("Code XML node should have a value attribute");
            let desc = code.first_element_child().unwrap().text().unwrap();
            let concept = concepts.get_mut(id).unwrap();
            concept.insert(key.to_string(), desc.to_string());
        }
    }
    Ok(concepts)
}

/// Parse H.4.1 fed XML data file to return an ordered map with a
/// balance sheet for each period of time.
pub fn parse_h41_data(text: &String) -> Result<ObservationMap, Box<dyn std::error::Error>> {
    let doc = Document::parse(text)?;
    let mut obs: ObservationMap = BTreeMap::new();

    let mut asset_series: Vec<Node<'_, '_>> = get_asset_series(&doc);
    let mut paths: Trie<String, String> = Trie::new();
    for serie in &mut asset_series {
        let serie_name = serie.attribute("SERIES_NAME").unwrap();
        let annotation = get_annotation(&serie);
        paths.insert(parse_annotation(&annotation), serie_name.to_string());
    }

    let bs_template = paths_to_balance_sheet_assets(paths);

    for serie in &mut asset_series {
        let annotation = get_annotation(&serie);
        let path = parse_annotation(&annotation);
        let mut observations = get_children_node_elements(&serie, FRB_NS, OBS_TAG);
        for observation in &mut observations {
            let date = NaiveDate::parse_from_str(
                observation.attribute("TIME_PERIOD").unwrap(),
                "%Y-%m-%d",
            )?;
            let value = if observation.attribute("OBS_STATUS") == Some("A") {
                observation.attribute("OBS_VALUE").unwrap()
            } else {
                "0"
            };
            obs.entry(date)
                .or_insert(bs_template.clone())
                .assets
                .update_concept_value(&path, value.parse::<i64>().unwrap_or(0));
        }
    }

    Ok(obs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annotation_to_path_test() {
        assert_eq!(
            "Assets",
            parse_annotation("Assets: Total Assets: Total assets: Wednesday level")
        );
        assert_eq!("Assets/Liquidity and Credit Facilities/Net portfolio holdings of Commercial Paper Funding Facility LLC",
            parse_annotation("Discontinued: Assets: Liquidity and Credit Facilities: Net portfolio holdings of Commercial Paper Funding Facility LLC: Wednesday level"));
    }
}
