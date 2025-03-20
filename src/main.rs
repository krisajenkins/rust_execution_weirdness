fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use vfs::{MemoryFS, VfsPath, VfsResult};

    #[test]
    fn dumb_test() -> VfsResult<()> {
        let root: VfsPath = MemoryFS::new().into();
        let path = root.join("somefile.txt")?;

        // This test case passes. Remove these braces and it will fail!
        {
            let mut file = path.create_file()?;
            file.write_all(b"Hello world\n")?;
        }

        let mut buf = String::new();
        path.open_file()?.read_to_string(&mut buf)?;
        assert_eq!("Hello world\n", buf);

        Ok(())
    }
}
