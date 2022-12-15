
pub fn solve_part_1(input: &Vec<String>) {

    let mut file_system = FileSystemNode::from_root();

    for row in input {

        match parse_string_for_command(row) {
            Ok(command) => {
                file_system.execute_command(command)
            },
            Err(msg) => panic!("{}", msg)
        }
    }
}

pub enum Command<'a> {
    ChangeDir(&'a str),
    ListChildren
}

#[derive(Debug, PartialEq)]
pub enum FileSystemNode {
    Dir(Vec<FileSystemNode>),
    File(u64),
}

impl FileSystemNode {
    fn from_root() -> FileSystemNode {
        FileSystemNode::Dir(vec![])
    }

    fn calculate_size(&self) -> u64 {
        match self {
            FileSystemNode::Dir(children) => {
                let mut size = 0;

                for node in children {
                    size += node.calculate_size()
                }

                size
            },
            FileSystemNode::File(size) => *size
        }
    }

    fn execute_command(&mut self, command: Command) {
        match command {
            Command::ChangeDir(arg) => {
                // check if arg name exists in dir

                // move into 
            },
            Command::ListChildren => {
                // advance line of input 
            
                // parse output into nodes as more of file system is "revealed"
                
            }
        }
    }
}

fn parse_string_for_command(s: &str) -> Result<Command, String>{
    let tokens = s.split_whitespace().collect::<Vec<&str>>();


    match tokens.first() {
        Some(token) => {
            match token {
                &"$" => {
                    match tokens[1] {
                        "cd" => {
                            Result::Ok(Command::ChangeDir(tokens[2]))
                        },
                        "ls" => {
                            Result::Ok(Command::ListChildren)
                        },
                        _ => Result::Err(format!("Input was not a command! Input was '{}'", s)),
                    }
                },
                _ => Result::Err(format!("Input was not a command! Input was '{}'", s)),
            }
        },
        None => Result::Err(format!("Could not read first from input! Input was '{}'", s))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file_system_size_calc() {
        let file_system = FileSystemNode::Dir(vec![
            FileSystemNode::Dir(vec![
                FileSystemNode::File(10001),
                FileSystemNode::File(10001),
                FileSystemNode::Dir(vec![
                    FileSystemNode::File(10001),
                ]),
                FileSystemNode::File(10001),
            ]),
            FileSystemNode::Dir(vec![
                FileSystemNode::File(10001),
            ]),
            FileSystemNode::File(10001)
        ]);

        assert_eq!(60006, file_system.calculate_size());
    }

    #[test]
    fn test_walking_file_tree() {
        let input = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir foo".to_string(),
            "dir bar".to_string(),
            "10001 image_1.tiff".to_string(),
            "$ cd bar".to_string(),
            "$ ls".to_string(),
            "10010 image_2.tiff".to_string(),
            "10010 image_3.tiff".to_string(),
            "$ cd ..".to_string(),
            "$ cd foo".to_string(),
            "$ ls".to_string(),
            "10010 image_4.tiff".to_string(),
            "10010 image_5.tiff".to_string(),
            "10010 image_6.tiff".to_string(),
        ];

        let mut fs = FileSystemNode::from_root();

        for row in input {
            match parse_string_for_command(&row) {
                Ok(command) => fs.execute_command(command),
                Err(s) => panic!("Uffda!")
            }
        }

        assert_eq!(
            FileSystemNode::Dir(
                vec![
                    FileSystemNode::Dir(vec![
                        FileSystemNode::File(10010),
                        FileSystemNode::File(10010),
                    ]),
                    FileSystemNode::Dir(vec![
                        FileSystemNode::File(10010),
                        FileSystemNode::File(10010),
                    ]),
                    FileSystemNode::File(10001),
                ]
            ),

            fs
        );
    }
}
