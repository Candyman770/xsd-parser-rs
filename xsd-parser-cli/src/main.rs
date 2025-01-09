use std::{
    fs,
    fs::OpenOptions,
    io::{prelude::*, Read},
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Parser;
use xsd_parser::{generator::builder::GeneratorBuilder, parser::{parse, types::{RsEntity, RsFile}}};
use std::collections::HashMap;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    /// Input .xsd file
    #[clap(long, short)]
    input: Option<PathBuf>,

    /// Output file
    #[clap(long, short)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = Opt::parse();

    let input_path = opt.input.unwrap_or_else(|| PathBuf::from("input/xsd"));
    let md = fs::metadata(&input_path).unwrap();
    if md.is_dir() {
        let output_path = opt.output.unwrap_or_else(|| PathBuf::from("output/rs"));
        process_dir_wrapper(&input_path, &output_path)?;
    } else {
        process_single_file(&input_path, opt.output.as_deref())?;
    }

    Ok(())
}

fn process_dir(input_path: &Path, output_path: &Path, rs_entities: &mut Vec<(PathBuf, Vec<RsEntity>)>) -> anyhow::Result<()> {
    // if !output_path.exists() {
    //     fs::create_dir_all(output_path)?;
    // }
    for entry in fs::read_dir(input_path)? {
        let path = entry?.path();
        if let Some(v) = path.file_name() {
            if let Some(v) = v.to_str() {
                if v.starts_with(".") {
                    continue;
                }
            }
        }
        if path.is_dir() {
            process_dir(&path, &PathBuf::from("dummy"), rs_entities)?;
        } else {
            // let output_file_path = PathBuf::from(path.file_name().unwrap()).with_extension("rs");
            // let output_file_path = output_path.join(output_file_path);
            // process_single_file(&path, Some(&output_file_path))?;
            let rs = get_rs_entities(&path)?;
            rs_entities.push((path, rs));
        }
    }
    Ok(())
}

fn process_dir_wrapper(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    let mut rs_entities = Vec::new();
    process_dir(input_path, output_path, &mut rs_entities)?;
    let mut count = HashMap::new();
    for (_, entities) in &rs_entities {
        for entity in entities {
            let name =
                match entity {
                    RsEntity::Struct(val) => Some(&val.name),
                    RsEntity::Enum(val) => Some(&val.name),
                    RsEntity::TupleStruct(val) => Some(&val.name),
                    _ => None,
                };
            if let Some(name) = name {
                count.entry(name.clone()).and_modify(|counter| *counter += 1).or_insert(1);
            }
        }
    }
    let mut common = HashMap::new();
    let mut counter = 0;
    for (input_path, entities) in rs_entities {
        let new_list = entities.into_iter().filter_map(|entity| {
            let name =
                match &entity {
                    RsEntity::Struct(val) => Some(&val.name),
                    RsEntity::Enum(val) => Some(&val.name),
                    RsEntity::TupleStruct(val) => Some(&val.name),
                    _ => None,
                };
            if let Some(name) = name {
                if count.get(name) > Some(&1) && (name.to_lowercase() != "document") {
                    common.insert(name.clone(), entity);
                    return None;
                }
            }
            Some(entity)
        }).collect();
        let output_path = PathBuf::from(input_path.file_name().as_ref().unwrap());
        process_single_file_new(new_list, &output_path.with_extension("rs"))?;
        counter += 1;
    }
    process_single_file_new(common.into_iter().map(|(_, v)| v).collect(), &PathBuf::from("common.rs"))?;
    Ok(())
}

fn process_single_file_new(types: Vec<RsEntity>, output_path: &Path) -> anyhow::Result<()> {
    let gen = GeneratorBuilder::default().build();
    let code: String = types.iter().map(|v| gen.generate(v)).collect();
    write_to_file(output_path, &code).context("Error writing file")?;
    Ok(())
}

fn get_rs_entities(input_path: &Path) -> anyhow::Result<Vec<RsEntity>> {
    let text = load_file(input_path)?;
    let rs = parse(text.as_str()).map_err(|_| anyhow::anyhow!("Error parsing file"))?;
    Ok(rs.types)
}

fn process_single_file(input_path: &Path, output_path: Option<&Path>) -> anyhow::Result<()> {
    let text = load_file(input_path)?;
    let rs_file = parse(text.as_str()).map_err(|_| anyhow::anyhow!("Error parsing file"))?;
    let gen = GeneratorBuilder::default().build();
    let code = gen.generate_rs_file(&rs_file);
    if let Some(output_filename) = output_path {
        write_to_file(output_filename, &code).context("Error writing file")?;
    } else {
        println!("{}", code);
    }
    Ok(())
}

fn load_file(path: &Path) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn write_to_file(path: &Path, text: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(path)?;
    file.write_all(text.as_bytes())
}
