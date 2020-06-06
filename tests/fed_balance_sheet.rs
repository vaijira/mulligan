use mulligan::fed;
use mulligan::NaiveDate;

const FED_XML_2020_DAT_PATH: &str = "tests/data/FRB_H41_2020.xml";

const ASSETS_20200527: &str = r#"
Assets                                                                   7097316
  Central Bank Liquidity Swaps                                      
    Central bank liquidity swaps                                          448946
  Liquidity and Credit Facilities                                   
    Corporate Credit Facilities LLC                                        34853
    Loans                                                                 106896
      Money Market Mutual Fund Liquidity Facility (Post 2020-03-18)        33244
      Payroll Protection Program Liquidity Facility                        49211
      Primary credit                                                       18198
      Primary dealer credit facility (Post 2020-03-17)                      6241
      Seasonal credit                                                          2
    Net portfolio holdings of Commercial Paper Funding Facility II L       12794
  Other Factors Supplying Reserve Balances                          
    Foreign currency denominated assets                                    20564
  Other                                                             
    Bank premises                                                           2205
    Coin                                                                    1447
    Gold certificate account                                               11037
    Items in process of collection                                            67
    Other Assets, Consolidated Table                                       26482
    Repurchase agreements                                                 181101
    Special drawing rights certificate account                              5200
  Securities Held Outright                                               5946969
    Federal agency debt securities                                          2347
    Mortgage-backed securities                                           1835110
    U.S. Treasury securities                                             4109512
      Bills                                                               326044
      Inflation compensation                                               36843
      Notes and bonds, inflation-indexed                                  257451
      Notes and bonds, nominal                                           3489174
  Unamortized discounts on securities held outright                        -5500
  Unamortized premiums on securities held outright                        304256
"#;

#[test]
fn balance_sheets_2020() {
    let h41_data_text = std::fs::read_to_string(FED_XML_2020_DAT_PATH).unwrap();
    let observations = fed::parse_h41_data(&h41_data_text).unwrap();
    let date = NaiveDate::parse_from_str("2020-05-27", "%Y-%m-%d").unwrap();
    
  let displayed_assets= format!("{}", observations.get(&date).unwrap().assets);
  println!("{}", displayed_assets);
  assert_eq!(ASSETS_20200527, displayed_assets);
}
