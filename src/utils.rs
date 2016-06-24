use std::path::Path;

use super::Result;

use super::parse;

use types::*;

pub fn find_features(root: &Path) -> Result<Features> {
    let mut result: Features = vec!();

    for entry in try!(root.read_dir()) {
        let entry = try!(entry);
        let path = entry.path();

        if try!(entry.file_type()).is_dir() {
            result.extend_from_slice(&try!(find_features(&path)));
        } else if path.extension().map_or(false, |ext| ext == "feature") {
            result.push(try!(parse(path.to_str().unwrap())));
        }
    }

    Ok(result)
}
