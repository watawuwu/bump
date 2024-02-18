use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use anyhow::Result;
use std::io;

#[allow(dead_code)]
pub fn mk_dir(path: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

#[allow(dead_code)]
pub fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

#[allow(dead_code)]
pub fn write_file(path: impl AsRef<Path>, contents: &[u8]) -> Result<()> {
    let file = File::create(path)?;
    let mut buf = BufWriter::new(file);
    buf.write_all(contents)?;
    Ok(())
}

pub fn read_from_stdin() -> Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {

    use crate::fs::*;
    use std::str;
    use tempfile::tempdir;

    #[test]
    fn mk_dir_one_layer_ok() -> Result<()> {
        let tmp_dir = tempdir()?;
        let path = tmp_dir.path().join("test");
        let actual = mk_dir(&path);
        assert!(actual.is_ok());
        assert!(path.exists());
        Ok(())
    }

    #[test]
    fn mk_dir_deep_layer_ok() -> Result<()> {
        let tmp_dir = tempdir()?;
        let path = tmp_dir.path().join("test/test/test/test");
        let actual = mk_dir(&path);
        assert!(actual.is_ok());
        assert!(path.exists());
        Ok(())
    }

    #[test]
    fn read_file_temp_ok() -> Result<()> {
        let expect = "hello";
        let file = "read_test.txt";

        let tmp_dir = tempdir()?;
        let tmp_file = tmp_dir.path().join(file);
        write_file(&tmp_file, expect.as_bytes()).unwrap();

        let actual = read_file(&tmp_file);
        assert!(actual.is_ok());
        assert_eq!(expect, str::from_utf8(&actual.unwrap()).unwrap());
        Ok(())
    }

    #[test]
    fn read_file_not_file_ng() -> Result<()> {
        let file = "not_found.txt";
        let tmp_dir = tempdir()?;
        let tmp_file = tmp_dir.path().join(file);
        let actual = read_file(tmp_file);
        assert!(actual.is_err());
        Ok(())
    }

    #[test]
    fn write_file_ok() -> Result<()> {
        let file = "write_test.txt";
        let tmp_dir = tempdir()?;
        let tmp_file = tmp_dir.path().join(file);
        let actual = write_file(tmp_file, b"write");
        assert!(actual.is_ok());
        Ok(())
    }
}
