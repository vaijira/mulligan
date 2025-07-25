use clap::{arg, Command};
use mulligan::fed;
use mulligan::fed::ObservationMap;
use mulligan::{Concept, ConceptType, NaiveDate};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::str;

const H41_FILE_PATH: &str = "/tmp/h41.zip";
const OBS_JSON_FILE_NAME: &str = "observations.json";
const ASSETS_CSV_FILE_NAME: &str = "assets.csv";
const LIABILITIES_CSV_FILE_NAME: &str = "liabilities.csv";
const CAPITAL_CSV_FILE_NAME: &str = "capital.csv";
const CSV_SEPARATOR_STR: &str = ",";

#[tokio::main]
async fn download_file(target: &str, dst_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut dest = {
        println!("Temporary file will be located under: '{dst_path:?}'");
        File::create(dst_path)?
    };

    let response = reqwest::get(target).await?;
    let bytes = response.bytes().await?;
    let mut data: &[u8] = bytes.as_ref();
    io::copy(&mut data, &mut dest)?;
    println!("Downloaded temporal file: '{dst_path:?}'");

    Ok(())
}

fn extract_zipfile(file_path: &str, prepend_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = Path::new(prepend_path).join(file.mangled_name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {i} extracted to \"{}\"", outpath.as_path().display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {i} extracted to \"{}\" ({} bytes)",
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn create_observation_json_file(
    dst_path: &str,
    obs: &ObservationMap,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&obs).unwrap();

    let mut dest = {
        println!("observations json file will be located under: '{dst_path:?}'");
        File::create(dst_path)?
    };

    io::copy(&mut json.as_bytes(), &mut dest)?;
    println!("observations json file created");

    Ok(())
}

fn csv_header(c: &Concept) -> Result<String, Box<dyn std::error::Error>> {
    let fields: Vec<String> = c
        .iter()
        .filter(|v| v.is_leaf())
        .map(|s| {
            let name = s.name();
            if name.contains(CSV_SEPARATOR_STR) {
                format!("\"{name}\"")
            } else {
                name.to_string()
            }
        })
        .collect();

    Ok(fields.join(CSV_SEPARATOR_STR))
}

fn csv_row(date: &NaiveDate, c: &Concept) -> Result<String, Box<dyn std::error::Error>> {
    let row_date = date.to_string();
    let row_values: Vec<String> = c
        .iter()
        .filter(|v| v.is_leaf())
        .map(|v| v.value.to_string())
        .collect();

    Ok(format!(
        "{}{}{}\n",
        row_date,
        CSV_SEPARATOR_STR,
        row_values.join(CSV_SEPARATOR_STR)
    ))
}

fn create_observation_csv_file(
    dst_path: &str,
    obs: &ObservationMap,
    ctype: &ConceptType,
) -> Result<(), Box<dyn std::error::Error>> {
    let header = csv_header(obs.iter().next().unwrap().1.get_concept(ctype))?;

    let mut dest = {
        println!("observations csv file will be located under: '{dst_path:?}'");
        File::create(dst_path)?
    };

    dest.write_all(b"date")?;
    dest.write_all(CSV_SEPARATOR_STR.as_bytes())?;
    dest.write_all(header.as_bytes())?;
    dest.write_all(b"\n")?;
    for (obs_date, obs_balance_sheet) in obs {
        let row = csv_row(obs_date, obs_balance_sheet.get_concept(ctype))?;
        io::copy(&mut row.as_bytes(), &mut dest)?;
    }

    println!("observations csv file created");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("fedparser")
        .version("0.2")
        .author("Jorge Perez Burgos <vaijira@gmail.com>")
        .about("Download H41 data from federal reserve website.")
        .arg(arg!(-o --output [OUTPUT_DIR] "Sets the output directory, default: /tmp"))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let output_dir = matches.value_of("OUTPUT_DIR").unwrap_or("./tmp");
    println!("Value for output dir: {output_dir}");

    if path_exists(output_dir) {
        println!("Directory {output_dir} already exists, skip downloading");
    } else {
        download_file(mulligan::fed::H41_FED_URL, H41_FILE_PATH)?;
        extract_zipfile(H41_FILE_PATH, output_dir)?;
    }

    let h41_data_file = format!("{output_dir}/{}", fed::H41_DATA_XML);
    let h41_data_text = std::fs::read_to_string(h41_data_file)?;
    let observations = fed::parse_h41_data(&h41_data_text)?;

    let obs_json_file = format!("{output_dir}/{OBS_JSON_FILE_NAME}");
    create_observation_json_file(&obs_json_file, &observations)?;

    let assets_csv_file = format!("{output_dir}/{ASSETS_CSV_FILE_NAME}");
    create_observation_csv_file(&assets_csv_file, &observations, &ConceptType::Assets)?;

    let liabilities_csv_file = format!("{output_dir}/{LIABILITIES_CSV_FILE_NAME}");
    create_observation_csv_file(
        &liabilities_csv_file,
        &observations,
        &ConceptType::Liabilities,
    )?;

    let capital_csv_file = format!("{output_dir}/{CAPITAL_CSV_FILE_NAME}");
    create_observation_csv_file(&capital_csv_file, &observations, &ConceptType::Capital)?;

    Ok(())
}
