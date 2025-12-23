#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Error, Read},
        path::Path,
        process::Command,
    };

    #[test]
    fn atcoder() -> Result<(), Error> {
        let dir_name = "test";
        for entry in fs::read_dir(dir_name)? {
            let entry = entry?;
            let file_type = entry.file_type().unwrap();
            if !file_type.is_file() {
                continue;
            }
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let ws: Vec<&str> = file_name.split('_').collect();
            if ws.len() != 4 || ws[2] != "in" {
                continue;
            }
            let task_name = format!("{}_{}", ws[0], ws[1]);
            println!("testing: {file_name}");
            let file = fs::File::open(&path)?;
            let output = Command::new("cargo")
                .arg("run")
                .arg("--bin")
                .arg(&task_name)
                .stdin(file)
                .output()?;
            let output = String::from_utf8_lossy(&output.stdout);
            let mut file = fs::File::open(
                Path::new(dir_name).join(&format!("{}_{}_out_{}", ws[0], ws[1], ws[3])),
            )?;
            let mut expected = String::new();
            file.read_to_string(&mut expected)?;
            assert!(output == expected);
        }
        Ok(())
    }
}
