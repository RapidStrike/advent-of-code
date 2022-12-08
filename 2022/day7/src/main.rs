/*
// Bro I was about to implement a whole-ass object
#[derive(Debug)]
struct ElfFile {
    filename: String,
    size: u64
}

#[derive(Debug)]
struct ElfFolder {
    name: String,
    files: Vec<ElfFile>,
    folders: HashMap<String, ElfFolder>,
    parent: Option<Box<RefCell<ElfFolder>>>
}

impl ElfFile {
    fn new(fname: String, fsize: u64) -> ElfFile {
        ElfFile {
            filename: fname,
            size: fsize
        }
    }
}

impl ElfFolder {
    fn new(fname: String) -> ElfFolder {
        ElfFolder {
            name: fname,
            files: Vec::<ElfFile>::new(),
            folders: HashMap::<String, ElfFolder>::new(),
            parent: None
        }
    }

    fn add_folder<'a>(&'a mut self, fname: String) {
        self.folders.insert(fname.clone(), ElfFolder::new(fname.clone()));
    }

    fn add_file<'a>(&'a mut self, fname: String, fsize: u64) {
        self.files.push(ElfFile::new(fname.clone(), fsize));
    }

    fn get_folder<'a>(&'a mut self, fname: String) -> Box<&ElfFolder> {
        return Box::new(self.folders.get(&fname).unwrap());
    }
}
 */

fn main() {
    let input_txt: &str = include_str!("../input.txt");
    let mut stack_a = Vec::<u64>::new();
    let mut stack_b = Vec::<u64>::new();
    let mut size_a: u64 = 0;
    let mut size_avail_b: u64 = 70000000;
    let mut size_req_b: u64 = 30000000;
    for l in input_txt.lines() {
        let cmd_line: Vec<&str> = l.split(" ").collect();
        if (cmd_line[0] == "$") & (cmd_line[1] == "cd") {
            // change directory
            if cmd_line[2] == ".." {
                // never looking in this folder again
                let dir_size = stack_a.pop().unwrap();
                if dir_size <= 100000 {
                    size_a += dir_size;
                }
                stack_b.push(dir_size);
                *stack_a.last_mut().unwrap() += dir_size;
            } else {
                // enter directory
                stack_a.push(0);
            }
        } else {
            // file read
            if (cmd_line[0] != "$") & (cmd_line[0] != "dir") {
                let siz: u64 = cmd_line[0].parse().unwrap();
                *stack_a.last_mut().unwrap() += siz;
            }
        }
    }
    while !stack_a.is_empty() {
        let dir_size = stack_a.pop().unwrap();
        if dir_size <= 100000 {
            size_a += dir_size;
        }
        stack_b.push(dir_size);
        if !stack_a.is_empty() {
            *stack_a.last_mut().unwrap() += dir_size;
        } else {
            size_avail_b -= dir_size;
            size_req_b -= size_avail_b;
        }
    }
    // all folders least to greatest
    stack_b.sort();
    for i in &stack_b {
        if *i > size_req_b {
            println!("Deleted Folder Size: {}", i);
            break;
        }
    }
    println!("Total Size: {}", size_a);
}

