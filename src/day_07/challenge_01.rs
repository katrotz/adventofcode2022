/*
--- Day 7: No Space Left On Device ---
You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its communication system. You try to run a system update:

$ system-update --please --pretty-please-with-sugar-on-top
Error: No space left on device
Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:

$ cd /
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
The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
cd / switches the current directory to the outermost directory, /.
ls means list. It prints out all of the files and directories immediately contained by the current directory:
123 abc means that the current directory contains a file named abc with size 123.
dir xyz means that the current directory contains a directory named xyz.
Given the commands and output in the example above, you can determine that the filesystem looks visually like this:

- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.

Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)

The total sizes of the directories above can be found as follows:

The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
Directory d has total size 24933642.
As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?
*/

const INPUT_FIXTURE_PATH: &str = "./src/day_07/fixtures/input.txt";

#[derive(Debug)]
pub struct FileSystem {
    sequence: usize,
    directories: Vec<Node<Directory>>,
    files: Vec<Node<File>>,
    current_dir: Option<NodeId>,
}

#[derive(Debug)]
pub struct DirSizeInfo {
    pub node_id: NodeId,
    pub size: i64
}

#[derive(Debug)]
pub struct Node<T> {
    id: NodeId,
    parent: Option<NodeId>,
    children: Option<Vec<NodeId>>,
    pub data: T,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct NodeId {
    index: usize,
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: i64,
}

impl File {
    pub fn new(name: String, size: i64) -> Self {
        Self { name, size }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        let mut fs = Self {
            sequence: 0,
            directories: Vec::new(),
            files: Vec::new(),
            current_dir: None
        };

        fs.new_dir_node(Directory { name: String::from("/") });

        fs
    }

    pub fn chdir(&mut self, node_id: NodeId) {
        self.current_dir = Some(node_id);
    }

    pub fn chdir_up(&mut self) {
        if let Some(current_dir_node_id) = self.current_dir.as_ref() {
            let current_dir = self.get_dir_by_node_id(current_dir_node_id);

            if let Some(parent_node_id) = current_dir.parent.as_ref() { self.chdir(NodeId { index: parent_node_id.index }) }
        }
    }

    fn node_id_seq(&mut self) -> NodeId {
        let index = self.sequence;

        self.sequence += 1;

        NodeId { index }
    }

    pub fn is_file_node_id(&self, node_id: &NodeId) -> bool {
        self.files.iter().any(|file_node| file_node.id.index == node_id.index)
    }

    pub fn get_dir_node_id_by_name(&mut self, dir_name: &str) -> Option<NodeId> {
        let node_maybe = self.directories.iter().find(|&dir| {
            if dir.parent.is_none() && self.current_dir.is_none() {
                return true;
            }

            dir.data.name == dir_name && dir.parent.unwrap().index == self.current_dir.unwrap().index
        });

        node_maybe.map(|node| node.id)
    }

    pub fn get_root_node_id(&self) -> Option<NodeId> {
        let node_maybe = self.directories.iter().find(|&dir| {
            dir.parent.is_none() && dir.data.name == "/"
        });

        node_maybe.map(|node| node.id)
    }

    pub fn new_dir_node(&mut self, data: Directory) -> NodeId {
        let node_id = self.node_id_seq();
        let index = node_id.index;

        self.directories.push(Node {
            id: node_id,
            parent: None,
            children: None,
            data,
        });

        NodeId { index }
    }

    pub fn new_file_node(&mut self, data: File) -> NodeId {
        let node_id = self.node_id_seq();

        self.files.push(Node {
            id: node_id,
            parent: None,
            children: None,
            data,
        });

        node_id
    }

    fn get_dir_by_node_id_mut(&mut self, node_id: &NodeId) -> &mut Node<Directory> {
        let dir_opt = self.directories.iter_mut().find(|dir| dir.id.index == node_id.index);

        match dir_opt {
            Some(dir) => dir,
            None => panic!("Directory not found")
        }
    }

    pub fn get_dir_by_node_id(&self, node_id: &NodeId) -> &Node<Directory> {
        let dir_opt = self.directories.iter().find(|dir| dir.id.index == node_id.index);

        match dir_opt {
            Some(dir) => dir,
            None => panic!("Directory not found {}", node_id.index)
        }
    }

    fn get_file_by_node_id_mut(&mut self, node_id: &NodeId) -> &mut Node<File> {
        let file_opt = self.files.iter_mut().find(|file| file.id.index == node_id.index);

        match file_opt {
            Some(file) => file,
            None => panic!("File not found")
        }
    }

    pub fn get_file_by_node_id(&self, node_id: &NodeId) -> &Node<File> {
        let file_opt = self.files.iter().find(|file| file.id.index == node_id.index);

        match file_opt {
            Some(file) => file,
            None => panic!("File not found")
        }
    }

    pub fn add_parent_dir(&mut self, parent_node_id: NodeId, child_node_id: NodeId) {
        if self.is_file_node_id(&child_node_id) {
            let child_node = self.get_file_by_node_id_mut(&child_node_id);
            child_node.parent = Some(parent_node_id);
        } else {
            let child_node = self.get_dir_by_node_id_mut(&child_node_id);
            child_node.parent = Some(parent_node_id);
        }

        let parent_dir = self.get_dir_by_node_id_mut(&parent_node_id);

        parent_dir.add_child_node(child_node_id);
    }

    pub fn node_size(&self, node_id: &NodeId) -> i64 {
        if self.is_file_node_id(node_id) {
            return self.get_file_by_node_id(node_id).data.size;
        }

        match self.get_dir_by_node_id(node_id).children.as_ref() {
            None => 0,
            Some(children) => children.to_vec().iter()
                .map(|child_node_id| self.node_size(child_node_id))
                .sum()
        }
    }

    pub fn fs_size_info(&self) -> Vec<DirSizeInfo> {
        self.directories.iter().map(|dir| DirSizeInfo {
            node_id: dir.id,
            size: self.node_size(&dir.id)
        }).collect::<Vec<DirSizeInfo>>()
    }

    // pub fn print_tree(&mut self) {
    //     let root_node = self.directories.iter().find(|&node| node.data.name == "/").unwrap();
    //
    //     println!("/ {}", self.node_size(&root_node.id));
    //
    //     if let Some(children) = root_node.children.as_ref() {
    //         children.iter().for_each(|child_node_id| self.print_node(child_node_id, 1));
    //     }
    // }
    //
    // fn print_node(&self, node_id: &NodeId, ident: usize) {
    //     if self.is_file_node_id(node_id) {
    //         let file = self.get_file_by_node_id(node_id);
    //         println!("{} |-- {} [{}]", "\t".repeat(ident), file.data.name, file.data.size / 1);
    //     } else {
    //         let dir = self.get_dir_by_node_id(node_id);
    //         println!("{} |-+ {} [{}]", "\t".repeat(ident), dir.data.name, self.node_size(&dir.id));
    //
    //         if let Some(children) = dir.children.as_ref() {
    //             children.iter().for_each(|child_node_id| {
    //                 self.print_node(child_node_id, ident + 1);
    //             });
    //         }
    //     }
    // }
}

impl Node<Directory> {
    pub fn add_child_node(&mut self, node_id: NodeId) {
        let children = self.children.as_mut();

        match children {
            None => {
                self.children = Some(vec![node_id]);
            }
            Some(children) => {
                children.push(node_id);
            }
        }
    }
}

pub struct Shell {}
impl Shell {
    pub fn is_change_directory_cmd(cmd: &str) -> bool {
        cmd.starts_with("$ cd ")
    }

    pub fn is_list_directory_cmd(cmd: &str) -> bool {
        cmd.eq("$ ls")
    }

    pub fn is_directory_list_item(cmd: &str) -> bool {
        cmd.starts_with("dir ")
    }

    pub fn is_file_list_item(cmd: &str) -> bool {
        !(Shell::is_change_directory_cmd(cmd) || Shell::is_list_directory_cmd(cmd) || Shell::is_directory_list_item(cmd))
    }

    pub fn get_change_directory_target(cmd: &str) -> String {
        cmd.strip_prefix("$ cd ").unwrap().to_string()
    }

    pub fn get_directory_list_item(cmd: &str) -> Directory {
        Directory {
            name: cmd.strip_prefix("dir ").unwrap().to_string()
        }
    }

    pub fn get_file_list_item(cmd: &str) -> File {
        let mut parts = cmd.split(' ');

        let size = parts.next().unwrap().parse::<i64>().unwrap();
        let name = parts.next().unwrap().to_string();

        File::new(name, size)
    }
}

pub fn solve() {
    let result = solve_challenge(INPUT_FIXTURE_PATH);

    assert_eq!("1334506", result);

    adventofcode2022::print_results(&result, "07", "01")
}

fn solve_challenge(fixture_path: &str) -> String {
    let fs = init_fs_from_fixture(fixture_path);

    let total_size: i64 = fs.directories.iter()
        .map(|node| node.id).collect::<Vec<NodeId>>().iter()
        .filter_map(|node_id| {
            let size = fs.node_size(node_id);

            if size > 100000 {
                return None;
            }

            Some(size)
        })
        .sum();

    // fs.print_tree();

    total_size.to_string()
}

pub fn init_fs_from_fixture(fixture_path: &str) -> FileSystem {
    let input = adventofcode2022::read_file_to_lines(fixture_path);
    let mut fs = FileSystem::new();

    for line in input {
        if Shell::is_change_directory_cmd(&line) {
            let target = Shell::get_change_directory_target(&line);

            if target == ".." {
                fs.chdir_up();
            } else if let Some(node_id) = fs.get_dir_node_id_by_name(&target) {
                fs.chdir(node_id);
            } else {
                panic!("Unexpected directory name")
            }
        }

        if Shell::is_directory_list_item(&line) {
            let target = Shell::get_directory_list_item(&line);
            let node_id = fs.new_dir_node(target);

            fs.add_parent_dir(fs.current_dir.unwrap(), node_id);
        }

        if Shell::is_file_list_item(&line) {
            let target = Shell::get_file_list_item(&line);
            let node_id = fs.new_file_node(target);

            fs.add_parent_dir(fs.current_dir.unwrap(), node_id);
        }
    }

    fs
}
