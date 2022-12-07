use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use anyhow::Context;

pub struct Parser {
    directory_sizes: HashMap<String, usize>,
}

impl Parser {
    pub fn new<R: Read>(buffer: BufReader<R>) -> anyhow::Result<Self> {
        let mut path = Vec::new();
        let mut directory_sizes = HashMap::new();

        for line in buffer.lines() {
            let line = line?;

            match line.starts_with('$') {
                true => {
                    // we're parsing a command
                    if line == "$ cd .." {
                        // we're going up a directory
                        path.pop();
                    } else if line.starts_with("$ cd ") {
                        let dir = line.replace("$ cd ", "");
                        path.push(dir);
                    }
                }
                false => {
                    // we're parsing a file or directory
                    if line.starts_with("dir") {
                        // this is a directory - we can ignore it
                    } else {
                        // this is a file - we need to parse it
                        let mut parts = line.split_whitespace();
                        // we only need the size - we can ignore the file name
                        let size = parts
                            .next()
                            .context("missing file size")?
                            .parse::<usize>()?;

                        // we need to add the size to all parent directories
                        // computing each parent directory on the fly is a bit wasteful
                        // but it's fast enough for this problem
                        for i in 0..path.len() {
                            let path_str = path[..=i].join("/");
                            let entry = directory_sizes.entry(path_str).or_insert(0);
                            *entry += size;
                        }
                    }
                }
            }
        }

        Ok(Self { directory_sizes })
    }

    pub fn get_all_by_top_limit(&self, top_limit: usize) -> usize {
        self.directory_sizes
            .values()
            .filter(|value| value <= &&top_limit)
            .sum()
    }

    pub fn get_one_by_free_space_required(
        &self,
        total_disk_space: usize,
        free_space_required: usize,
    ) -> anyhow::Result<usize> {
        let current_space_usage = self
            .directory_sizes
            .get("/")
            .context("failed to find the root directory")?;

        let current_free_space = total_disk_space - current_space_usage;

        if current_free_space >= free_space_required {
            return Ok(0);
        }

        let extra_free_space_required = free_space_required - current_free_space;

        let mut values = self
            .directory_sizes
            .values()
            .filter(|value| value >= &&extra_free_space_required)
            .collect::<Vec<_>>();

        values.sort();
        let value = **values
            .get(0)
            .context("failed to find a directory large enough")?;

        Ok(value)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    #[test]
    fn test_parser_get_all_by_top_limit() {
        let reader = BufReader::new(INPUT.as_bytes());
        let parser = Parser::new(reader).unwrap();

        assert_eq!(parser.get_all_by_top_limit(100000), 95437);
    }

    #[test]
    fn test_parser_get_one_by_free_space_required() {
        let reader = BufReader::new(INPUT.as_bytes());
        let parser = Parser::new(reader).unwrap();

        assert_eq!(
            parser
                .get_one_by_free_space_required(70000000, 30000000)
                .unwrap(),
            24933642
        );
    }
}
