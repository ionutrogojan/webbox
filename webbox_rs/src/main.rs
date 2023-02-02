use std::env;
use std::io::Write;
use std::path::Path;
use std::fs::{ self, File, canonicalize };
use::std::process:: { self, Command };
use std::{ time::Duration, thread::sleep };
// external
use webbrowser;

fn main() {
    println!("\nRunning \x1b[1;35mnew\x1b[0m \x1b[1;94mWebbox\x1b[0m!");
    let content = "https://github.com/ionutrogojan/webbox\nhttps://www.rust-lang.org/\nhttps://dictionary.cambridge.org/us/dictionary/english/blazingly\n";
    let args: Vec<String> = env::args().collect();
    // config file path -> Unix
    let path = Path::new(".webbox/config.webbox");
    // links list
    let config = fs::read_to_string(path).expect("\x1b[1;31mError\x1b[0m: Failed to read config file\n");

    match args.len() {
        1 => {
            // argument message
            println!("\x1b[1;35mNo\x1b[0m commands passed, running \x1b[1;94mdefault\x1b[0m process\n");
            // check file
            if path.exists() { println!("\"\x1b[1;35mconfig.webbox\x1b[0m\" file \x1b[0;93mexists.\x1b[0m\n") }
            else { w_update_file(path, content, "created") }
            // loop thourgh the links and open them at 50 milliseconds apart to avoid unordered execution
            for link in config.lines() { open_link(link, 50) }
            // empty line for looks
            println!("");
            // macOS hack to prevent hanging terminal 
            macos_exit();
        },
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "-help" => output_help(cmd),
                "-list" => output_list(cmd, path, config),
                "-config-path" => output_path(cmd, path),
                // invalid command
                _ => eprintln!("\x1b[1;31mError\x1b[0m: Invalid command \x1b[1;35m{}\x1b[0m\nUse \x1b[1;94m-help\x1b[0m to get a list of available commands\n", cmd)
            }
        },
        3 => {
            let cmd = &args[1];
            let arg = &args[2];
            match &cmd[..] {
                "-new-link" => output_new(cmd, path, arg, config),
                "-remove-link" => {
                    let index = arg.parse().expect("\x1b[1;31mError\x1b[0m: \x1b[1;35m[index]\x1b[0m is not a positive integer\n");
                    if path.exists() {
                        // initialize the list
                        let mut list =  vec![];
                        // push each link inside the list
                        for link in config.lines() { list.push(link) }
                        // is the index in range
                        if index < list.len() {
                            // remove the link at index
                            list.remove(index);
                            // join the links back to a string
                            let content = list.join("\n") + "\n";
                            // update the file
                            w_update_file(path, &content, "updated");
                        }
                        else { eprintln!("\x1b[1;31mError\x1b[0m: index[\x1b[1;35m{}\x1b[0m] is not inside the list\nUse \x1b[1;94m-list\x1b[0m to get a list of available indexes\n", index) }
                    }
                },
                _ => eprintln!("\x1b[1;31mError\x1b[0m: Invalid command or argument \nUse \x1b[1;94m-help\x1b[0m to get a list of available commands\n")
            }
        },
        4 => {
            let cmd = &args[1];
            let index: usize = args[2].parse().expect("\x1b[1;31mError\x1b[0m: \x1b[1;35m[index]\x1b[0m is not a positive integer\n");
            let arg = &args[3];
            match &cmd[..] {
                "-update-link" => {
                    if path.exists() {
                        // initialize the list
                        let mut list =  vec![];
                        // push each link inside the list
                        for link in config.lines() { list.push(link) }
                        // is the index in range
                        if index < list.len() {
                            // update the link at index
                            list[index] = arg;
                            // join the links back to a string
                            let content = list.join("\n") + "\n";
                            // update the file
                            w_update_file(path, &content, "updated");
                        }
                        else { eprintln!("\x1b[1;31mError\x1b[0m: \x1b[1;35m{}\x1b[0m is not inside the list\nUse \x1b[1;94m-list\x1b[0m to get a list of available indexes\n", index) }
                    }
                },
                "-switch-link" => {
                    if path.exists() {
                        let index2: usize = arg.parse().expect("\x1b[1;31mError\x1b[0m: \x1b[1;35m[index_1]\x1b[0m is not a positive integer\n");
                        // initialize the list
                        let mut list =  vec![];
                        // push each link inside the list
                        for link in config.lines() { list.push(link) }
                        // is the index in range
                        if index < list.len() && index2 < list.len() {
                            // temp hold index link
                            let temp = list[index];
                            // update the link at index with link at index2
                            list[index] = list[index2];
                            // update the link at index 2 with the link at index
                            list[index2] = temp;
                            // join the links back to a string
                            let content = list.join("\n") + "\n";
                            // update the file
                            w_update_file(path, &content, "updated");
                        }
                        else { eprintln!("\x1b[1;31mError\x1b[0m: index[\x1b[1;35m{}\x1b[0m] or index[\x1b[1;35m{}\x1b[0m] is not inside the list\nUse \x1b[1;94m-list\x1b[0m to get a list of available indexes\n", index, index2) }
                    }
                },
                // invalid command
                _ => eprintln!("\x1b[1;31mError\x1b[0m: Invalid command or argument \nUse \x1b[1;94m-help\x1b[0m to get a list of available commands\n")   
            }
        },
        // too many arguments
        _ => eprintln!("\x1b[1;31mError\x1b[0m: Too many commands\nUse \x1b[1;94m-help\x1b[0m to get a list of available commands\n")
    }
}

fn output_help(cmd: &String) {
    // help sausage 
    println!("\x1b[1;35m{}\x1b[0m command passed, running \x1b[1;94m{}\x1b[0m process\n\nRunning \x1b[1;94mWebbox\x1b[0m without commands will activate the main process \x1b[1;32m(opening the tabs)\x1b[0m\n\nHere are the available commands for \x1b[1;94mWebbox\x1b[0m!:\n\x1b[1;35m-help\x1b[0m : list of arguments | eg : ./webbox -help\n\x1b[1;35m-list\x1b[0m : list the existing links | eg : ./webbox -list\n\x1b[1;35m-new-link\x1b[0m \x1b[1;94m[link]\x1b[0m : add link to list | eg : ./webbox -new-link https://www.example.com\n\x1b[1;35m-update-link\x1b[0m \x1b[1;32m[index]\x1b[0m \x1b[1;94m[link]\x1b[0m : update existing link | eg : ./webbox -update-link 2 https://www.example.com\n\x1b[1;35m-remove-link\x1b[0m \x1b[1;32m[index]\x1b[0m : remove existing link | eg : ./webbox -remove-link 2\n\x1b[1;35m-switch-link\x1b[0m \x1b[1;32m[index]\x1b[0m \x1b[1;32m[index]\x1b[0m : switch link position with another link | eg: -switch-link 2 1\n\x1b[1;35m-config-path\x1b[0m : config file location | eg : ./webbox -config-path\n", cmd, cmd);
}

fn output_list(cmd: &String, file: &Path, config: String) {
    // argument message
    println!("\x1b[1;35m{}\x1b[0m command passed, running \x1b[1;94m{}\x1b[0m process\n", cmd, cmd);
    if file.exists() {
        // check confirmation
        println!("\"\x1b[1;35mconfig.webbox\x1b[0m\" file \x1b[0;93mexists.\x1b[0m\n");
        // enumerate throught the links
        for (index, link) in config.lines().enumerate() { println!("\x1b[1;35m{}\x1b[0m : \x1b[1;32m{}\x1b[0m", index, link) }
        // empty line for looks
        println!("");
    }
    else { eprintln!("\x1b[1;31mError\x1b[0m: Missing file \"\x1b[1;35mconfig.webbox\x1b[0m\"\n") }
}

fn output_path(cmd: &String, path: &Path) {
    // argument message
    println!("\x1b[1;35m{}\x1b[0m command passed, running \x1b[1;94m{}\x1b[0m process\n", cmd, cmd);
    // print config.webbox path
    println!("{}\n", canonicalize(path).expect("\x1b[1;31mError\x1b[0m: Could not find the config file\nRun the program at least once in order to create it\n").display())
}

fn output_new(cmd: &String, path: &Path, arg: &String, config: String) {
    // argument message
    println!("\x1b[1;35m{}\x1b[0m command passed, running \x1b[1;94m{}\x1b[0m process\n", cmd, cmd);
    // add the new link to the string
    let content = config + arg + "\n";
    // update the file with the new text
    w_update_file(path, &content, "updated");
}

fn macos_exit() {
    // spawn "kill" process to close terminal after execution on macos
    if cfg!(target_os = "macos") { Command::new("kill").arg("Terminal").spawn().expect("terminal closing failed"); }
    process::exit(0);
}

fn open_link(link: &str, ms: u64) {
    // if the link can open print success
    if webbrowser::open(link).is_ok() { println!("\x1b[1;32mSuccess\x1b[0m opening \x1b[1;35m{}\x1b[0m", link) }
    // if the link can not open print error
    else { eprintln!("\n\x1b[1;31mError\x1b[0m opening \x1b[1;33m{}\x1b[0m", link) }
    // sleep to prevent pages from opening in an unordered manner
    sleep(Duration::from_millis(ms));
}

fn w_update_file(path: &Path, content: &str, process: &str) {
    // check for main folder
    let folder = Path::new(".webbox");
    // create main folder
    if !folder.exists() {
        match fs::create_dir(".webbox") {
            Ok(_) => println!("created folder"),
            Err(e) => panic!("\n\x1b[1;31mError\x1b[0m creating folder :\n \x1b[1;33m{}\x1b[0m.\n", e)
        }
    }
    // create file
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => panic!("\n\x1b[1;31mError\x1b[0m creating file {} :\n \x1b[1;33m{}\x1b[0m.\n", path.display(), e)
    };
    // write contents to file
    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("\"\x1b[1;35mconfig.webbox\x1b[0m\" file \x1b[0;93m{}.\x1b[0m\n", process),
        Err(e) => panic!("\n\x1b[1;31mError\x1b[0m writing file {} :\n \x1b[1;33m{}\x1b[0m.\n", path.display(), e)
    };
}

fn _output_message(cmd: &String) {
    println!("\x1b[1;35m{}\x1b[0m command passed, running \x1b[1;94m{}\x1b[0m process\n\nThis command is yet to be implemented in future iterations\n", cmd, cmd)
}