use byteorder::{LittleEndian, ReadBytesExt};
use serde::Deserialize;
use std::io::{Error, ErrorKind, Read};
use std::fs::{self, File};
use std::time::Instant;
use std::path::PathBuf;

// from: https://x4fx77x4f.github.io/dennispedia/teardown/tdpth.html
#[derive(Deserialize)]
struct TDPath {
    magic: [u8; 5],   // TDPTH
    version: [u8; 3], // major, minor, patch
    time: f32,        // length of path in seconds
    length: u32,      // number of nodes
    nodes: Vec<Node>,
}

#[derive(Deserialize, Debug)]
struct Node {
    time: f32,  // timestamp in seconds
    flags: u32, // unknown; either 0 or 1
    pos: [f32; 3],
}

impl TDPath {
    fn deserialize(file_path: &PathBuf) -> Result<Self, std::io::Error> {
        let mut file = File::open(file_path)?;

        let mut magic = [0; 5];
        file.read_exact(&mut magic)?;

        if &magic != b"TDPTH" {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid magic number"));
        }

        let mut version = [0; 3];
        file.read_exact(&mut version)?;

        let time = file.read_f32::<LittleEndian>()?;
        let length = file.read_u32::<LittleEndian>()?;

        let mut nodes = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let time = file.read_f32::<LittleEndian>()?;
            let flags = file.read_u32::<LittleEndian>()?;
            let mut pos = [0.0; 3];
            file.read_f32_into::<LittleEndian>(&mut pos)?;
            nodes.push(Node { time, flags, pos });
        }

        Ok(TDPath {
            magic,
            version,
            time,
            length,
            nodes,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_data = std::env::var("LOCALAPPDATA")?;
    let td_folder = PathBuf::from(&app_data).join("Teardown");

    for dir in fs::read_dir(td_folder)? {
        let path = dir?.path();

        if path.is_dir() {
            continue;
        }

        if path.extension().unwrap() == "pth" {
            let now = Instant::now();
            println!("Converting: {:?}", path);

            let tdp = TDPath::deserialize(&path)?;

            // build the lua file
            let mut lua = String::from("path = {\n");

            lua.push_str(&format!("\tversion = '{}.{}.{}',\n", tdp.version[0], tdp.version[1], tdp.version[2]));
            lua.push_str(&format!("\ttime = {},\n", tdp.time));
            lua.push_str(&format!("\tlength = {},\n", tdp.length));
            lua.push_str("\tnodes = {\n");

            for node in tdp.nodes {
                lua.push_str(&format!("\t\t{{ {}, {}, {} }},\n", node.pos[0], node.pos[1], node.pos[2]));
            }
            lua.push_str("\t}\n}\n");

            // write the file
            let mut lua_path = PathBuf::from(path.file_stem().unwrap());
            lua_path.set_extension("lua");

            fs::write(lua_path, lua)?;

            println!("Converted to lua in {:?}", now.elapsed());
        }
    }

    Ok(())
}