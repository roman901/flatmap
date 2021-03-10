use regex::Regex;
use std::fs;
use std::io::BufReader;
use std::path::Path;
// use zip;
// use zip::result::ZipError;

pub struct MCMod {}
impl MCMod {
    pub fn from_file(path: &Path) {
        /*let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut archive = zip::ZipArchive::new(reader).unwrap();

        let asset_re = Regex::new(r"assets/(\w+)/blockstates/(\w+).json").unwrap();

        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => path,
                None => {
                    continue;
                }
            };

            /*let name = file.name();
            if asset_re.is_match(name) {
                println!(
                    "Entry {} is a directory with name \"{}\"",
                    i,
                    outpath.display()
                );
            }*/
        }
        Ok(MCMod{})*/
    }
}

struct MCPack {}
