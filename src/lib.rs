#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Error, Read},
    };

    #[test]
    fn atcoder() -> Result<(), Error> {
        for entry in fs::read_dir("test")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                println!("ファイル名: {:?}", path.file_name().unwrap());
                let mut file = fs::File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                println!("内容:\n{}", contents);
            }
        }
        Ok(())
    }
}
