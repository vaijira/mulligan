use mulligan::fed;
use mulligan::{ConceptType, NaiveDate};

const FED_XML_2020_DATA_PATH: &str = "tests/data/FRB_H41_2020.xml";
const FED_XML_2010_DATA_PATH: &str = "tests/data/FRB_H41_2010.xml";
const FED_XML_2006_DATA_PATH: &str = "tests/data/FRB_H41_2006.xml";
const LINE_SEPARATOR_STR: &str = "\n";

fn assert_by_lines(lines1: &str, lines2: &str) {
    for (l1, l2) in lines1
        .split(LINE_SEPARATOR_STR)
        .zip(lines2.split(LINE_SEPARATOR_STR))
    {
        assert_eq!(l1, l2);
    }
}

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
"#;

const LIABILITIES_20200527: &str = r#"
Liabilities                                                              7058402
  Deferred availability cash items                                           355
  Deposits                                                               4839897
    Foreign official                                                       16251
    Other                                                                 179062
    Other deposits held by depository institutions                       3317688
    U.S. Treasury, General Account                                       1326897
  Earnings remittances due to the U.S. Treasury                             1160
  Federal Reserve notes, net of F.R. Bank holdings                       1899514
  Other liabilities and accrued dividends (Includes the liability fo        8160
  Reverse repurchase agreements                                           243976
    Foreign official and international accounts                           239650
    Others                                                                  4326
"#;

const ASSETS_20200520: &str = r#"
Assets                                                                   7037258
  Central Bank Liquidity Swaps                                      
    Central bank liquidity swaps                                          446103
  Liquidity and Credit Facilities                                   
    Corporate Credit Facilities LLC                                         1801
    Loans                                                                 108577
      Money Market Mutual Fund Liquidity Facility (Post 2020-03-18)        36449
      Payroll Protection Program Liquidity Facility                        45090
      Primary credit                                                       19535
      Primary dealer credit facility (Post 2020-03-17)                      7501
      Seasonal credit                                                          3
    Net portfolio holdings of Commercial Paper Funding Facility II L        4293
  Other                                                             
    Bank premises                                                           2206
    Coin                                                                    1478
    Gold certificate account                                               11037
    Items in process of collection                                            51
    Other Assets, Consolidated Table                                       25635
    Repurchase agreements                                                 157351
    Special drawing rights certificate account                              5200
  Securities Held Outright                                               5954518
    Federal agency debt securities                                          2347
    Mortgage-backed securities                                           1862841
    U.S. Treasury securities                                             4089331
      Bills                                                               326044
      Inflation compensation                                               36797
      Notes and bonds, inflation-indexed                                  255266
      Notes and bonds, nominal                                           3471224
"#;

const LIABILITIES_20200520: &str = r#"
Liabilities                                                              6998365
  Deferred availability cash items                                           261
  Deposits                                                               4780812
    Foreign official                                                       16228
    Other                                                                 267066
    Other deposits held by depository institutions                       3304221
    U.S. Treasury, General Account                                       1193297
  Earnings remittances due to the U.S. Treasury                             1530
  Federal Reserve notes, net of F.R. Bank holdings                       1890000
  Other liabilities and accrued dividends (Includes the liability fo       11644
  Reverse repurchase agreements                                           266649
    Foreign official and international accounts                           256923
    Others                                                                  9726
"#;

const ASSETS_20200513: &str = r#"
Assets                                                                   6934227
  Central Bank Liquidity Swaps                                      
    Central bank liquidity swaps                                          440934
  Liquidity and Credit Facilities                                   
    Corporate Credit Facilities LLC                                          305
    Loans                                                                 114927
      Money Market Mutual Fund Liquidity Facility (Post 2020-03-18)        39820
      Payroll Protection Program Liquidity Facility                        40580
      Primary credit                                                       24239
      Primary dealer credit facility (Post 2020-03-17)                     10288
    Net portfolio holdings of Commercial Paper Funding Facility II L        4292
  Other                                                             
    Bank premises                                                           2205
    Coin                                                                    1509
    Gold certificate account                                               11037
    Items in process of collection                                            49
    Other Assets, Consolidated Table                                       40394
    Repurchase agreements                                                 157354
    Special drawing rights certificate account                              5200
  Securities Held Outright                                               5843376
    Federal agency debt securities                                          2347
    Mortgage-backed securities                                           1783761
    U.S. Treasury securities                                             4057268
      Bills                                                               326044
      Inflation compensation                                               36592
      Notes and bonds, inflation-indexed                                  252016
      Notes and bonds, nominal                                           3442616
"#;

const LIABILITIES_20200513: &str = r#"
Liabilities                                                              6895336
  Deferred availability cash items                                           288
  Deposits                                                               4663966
    Foreign official                                                       16328
    Other                                                                 246034
    Other deposits held by depository institutions                       3263431
    U.S. Treasury, General Account                                       1138172
  Earnings remittances due to the U.S. Treasury                             2095
  Federal Reserve notes, net of F.R. Bank holdings                       1881319
  Other liabilities and accrued dividends (Includes the liability fo       21114
  Reverse repurchase agreements                                           281150
    Foreign official and international accounts                           267325
    Others                                                                 13825
"#;

const ASSETS_20200506: &str = r#"
Assets                                                                   6721420
  Central Bank Liquidity Swaps                                      
    Central bank liquidity swaps                                          444885
  Liquidity and Credit Facilities                                   
    Loans                                                                 113342
      Money Market Mutual Fund Liquidity Facility (Post 2020-03-18)        42763
      Payroll Protection Program Liquidity Facility                        29181
      Primary credit                                                       26494
      Primary dealer credit facility (Post 2020-03-17)                     14903
    Net portfolio holdings of Commercial Paper Funding Facility II L        3988
  Other                                                             
    Bank premises                                                           2203
    Coin                                                                    1557
    Gold certificate account                                               11037
    Items in process of collection                                            30
    Other Assets, Consolidated Table                                       37069
    Repurchase agreements                                                 172700
    Special drawing rights certificate account                              5200
  Securities Held Outright                                               5627918
    Federal agency debt securities                                          2347
    Mortgage-backed securities                                           1605380
    U.S. Treasury securities                                             4020191
      Bills                                                               326044
      Inflation compensation                                               36365
      Notes and bonds, inflation-indexed                                  248266
      Notes and bonds, nominal                                           3409516
"#;

const LIABILITIES_20200506: &str = r#"
Liabilities                                                              6682549
  Deferred availability cash items                                           272
  Deposits                                                               4518539
    Foreign official                                                       16336
    Other                                                                 193329
    Other deposits held by depository institutions                       3165606
    U.S. Treasury, General Account                                       1143268
  Earnings remittances due to the U.S. Treasury                             2029
  Federal Reserve notes, net of F.R. Bank holdings                       1873325
  Other liabilities and accrued dividends (Includes the liability fo       25208
  Reverse repurchase agreements                                           265206
    Foreign official and international accounts                           264031
    Others                                                                  1175
"#;

const ASSETS_20200429: &str = r#"
Assets                                                                   6655929
  Central Bank Liquidity Swaps                                      
    Central bank liquidity swaps                                          438953
  Liquidity and Credit Facilities                                   
    Loans                                                                 123028
      Money Market Mutual Fund Liquidity Facility (Post 2020-03-18)        46277
      Payroll Protection Program Liquidity Facility                        19488
      Primary credit                                                       31759
      Primary dealer credit facility (Post 2020-03-17)                     25504
    Net portfolio holdings of Commercial Paper Funding Facility II L        3372
  Other                                                             
    Bank premises                                                           2208
    Coin                                                                    1598
    Gold certificate account                                               11037
    Items in process of collection                                            41
    Other Assets, Consolidated Table                                       35749
    Repurchase agreements                                                 158202
    Special drawing rights certificate account                              5200
  Securities Held Outright                                               5578486
    Federal agency debt securities                                          2347
    Mortgage-backed securities                                           1604720
    U.S. Treasury securities                                             3971419
      Bills                                                               326044
      Inflation compensation                                               36038
      Notes and bonds, inflation-indexed                                  242290
      Notes and bonds, nominal                                           3367047
"#;

const LIABILITIES_20200429: &str = r#"
Liabilities                                                              6617091
  Deferred availability cash items                                          1438
  Deposits                                                               4460138
    Foreign official                                                       16323
    Other                                                                 204070
    Other deposits held by depository institutions                       3163513
    U.S. Treasury, General Account                                       1076232
  Earnings remittances due to the U.S. Treasury                             1933
  Federal Reserve notes, net of F.R. Bank holdings                       1862131
  Other liabilities and accrued dividends (Includes the liability fo       24279
  Reverse repurchase agreements                                           269106
    Foreign official and international accounts                           267656
    Others                                                                  1450
"#;

#[test]
fn balance_sheets_2020() {
    let h41_data_text = std::fs::read_to_string(FED_XML_2020_DATA_PATH).unwrap();
    let observations = fed::parse_h41_data(&h41_data_text).unwrap();

    let date = NaiveDate::parse_from_str("2020-05-27", "%Y-%m-%d").unwrap();
    let displayed_assets = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Assets));
    assert_by_lines(ASSETS_20200527, &displayed_assets);
    let displayed_liabilities = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Liabilities));
    assert_by_lines(LIABILITIES_20200527, &displayed_liabilities);

    let date = NaiveDate::parse_from_str("2020-05-20", "%Y-%m-%d").unwrap();
    let displayed_assets = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Assets));
    assert_by_lines(ASSETS_20200520, &displayed_assets);
    let displayed_liabilities = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Liabilities));
    assert_by_lines(LIABILITIES_20200520, &displayed_liabilities);

    let date = NaiveDate::parse_from_str("2020-05-13", "%Y-%m-%d").unwrap();
    let displayed_assets = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Assets));
    assert_by_lines(ASSETS_20200513, &displayed_assets);
    let displayed_liabilities = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Liabilities));
    assert_by_lines(LIABILITIES_20200513, &displayed_liabilities);

    let date = NaiveDate::parse_from_str("2020-05-06", "%Y-%m-%d").unwrap();
    let displayed_assets = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Assets));
    assert_by_lines(ASSETS_20200506, &displayed_assets);
    let displayed_liabilities = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Liabilities));
    assert_by_lines(LIABILITIES_20200506, &displayed_liabilities);

    let date = NaiveDate::parse_from_str("2020-04-29", "%Y-%m-%d").unwrap();
    let displayed_assets = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Assets));
    assert_by_lines(ASSETS_20200429, &displayed_assets);
    let displayed_liabilities = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Liabilities));
    assert_by_lines(LIABILITIES_20200429, &displayed_liabilities);
}

const ASSETS_20100310: &str = r#"
Assets                                                                   2282548
  Liquidity and Credit Facilities                                   
    Loans                                                                  83791
      Credit extended to American International Group, Inc., Net           24845
      Primary credit                                                       13778
      Seasonal credit                                                         10
      Secondary credit                                                       700
      Term Asset-Backed Securities Loan Facility                           44458
    Net portfolio holdings of Commercial Paper Funding Facility LLC         7757
    Term auction credit                                                    15425
  Net Portfolio Holdings of Maiden Lane LLCs                        
    Net portfolio holdings of Maiden Lane II LLC                           15331
    Net portfolio holdings of Maiden Lane III LLC                          22118
    Net portfolio holdings of Maiden Lane LLC                              27267
  Net Portfolio Holdings of TALF LLC                                
    Net portfolio holdings of TALF LLC                                       372
  Other                                                             
    Bank premises                                                           2238
    Coin                                                                    2133
    Gold certificate account                                               11037
    Items in process of collection                                           373
    Other Assets, Consolidated Table                                       14888
    Special drawing rights certificate account                              5200
  Preferred Interests                                               
    Preferred interests in AIA Aurora LLC and ALICO Holdings LLC           25106
  Securities Held Outright                                               1974773
    Federal agency debt securities                                        169011
    Mortgage-backed securities                                           1029172
    U.S. Treasury securities                                              776591
      Bills                                                                18423
      Inflation compensation                                                5519
      Notes and bonds, inflation-indexed                                   43777
      Notes and bonds, nominal                                            708872
"#;

const LIABILITIES_20100310: &str = r#"
Liabilities                                                              2229730
  Deferred availability cash items                                          2391
  Deposits                                                               1266950
    Foreign official                                                        2616
    Other                                                                    295
    Other deposits held by depository institutions                       1190756
    U.S. Treasury, General Account                                         23292
    U.S. Treasury, Supplementary Financing Account                         49993
  Federal Reserve notes, net of F.R. Bank holdings                        893623
  Other liabilities and accrued dividends (Includes the liability fo       10862
  Reverse repurchase agreements                                            55903
    Foreign official and international accounts                            55903
"#;

#[test]
fn balance_sheets_2010() {
    let h41_data_text = std::fs::read_to_string(FED_XML_2010_DATA_PATH).unwrap();
    let observations = fed::parse_h41_data(&h41_data_text).unwrap();

    let date = NaiveDate::parse_from_str("2010-03-10", "%Y-%m-%d").unwrap();
    let displayed_assets = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Assets));
    assert_by_lines(ASSETS_20100310, &displayed_assets);
    let displayed_liabilities = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Liabilities));
    assert_by_lines(LIABILITIES_20100310, &displayed_liabilities);
}

const ASSETS_20060308: &str = r#"
Assets                                                                    840528
  Liquidity and Credit Facilities                                   
    Loans                                                                     43
      Seasonal credit                                                         43
  Other                                                             
    Bank premises                                                           1821
    Coin                                                                     820
    Gold certificate account                                               11040
    Items in process of collection                                          9405
    Other Assets, Consolidated Table                                        8299
    Repurchase agreements                                                  26500
    Special drawing rights certificate account                              2200
  Securities Held Outright                                                755576
    U.S. Treasury securities                                              755576
      Bills                                                               274142
      Inflation compensation                                                3160
      Notes and bonds, inflation-indexed                                   22196
      Notes and bonds, nominal                                            456077
"#;

const LIABILITIES_20060308: &str = r#"
Liabilities                                                               812877
  Deferred availability cash items                                          8175
  Deposits                                                                 25722
    Foreign official                                                          86
    Other                                                                    230
    Other deposits held by depository institutions                         20556
    U.S. Treasury, General Account                                          4851
  Federal Reserve notes, net of F.R. Bank holdings                        753788
  Other liabilities and accrued dividends (Includes the liability fo        1382
  Reverse repurchase agreements                                            23810
    Foreign official and international accounts                            23810
"#;

#[test]
fn balance_sheets_2006() {
    let h41_data_text = std::fs::read_to_string(FED_XML_2006_DATA_PATH).unwrap();
    let observations = fed::parse_h41_data(&h41_data_text).unwrap();

    let date = NaiveDate::parse_from_str("2006-03-08", "%Y-%m-%d").unwrap();
    let displayed_assets = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Assets));
    assert_by_lines(ASSETS_20060308, &displayed_assets);
    let displayed_liabilities = format!("{}", observations.get(&date).unwrap().get_concept(&ConceptType::Liabilities));
    assert_by_lines(LIABILITIES_20060308, &displayed_liabilities);
}
