use std::env;
use std::io::Write;
use std::fs::{ self, File };
use std::path::{ Path, PathBuf };
use::std::process:: { self, Command };
use std::{ time::Duration, thread::sleep };
// external
use dirs;
use webbrowser;

fn main() {
    println!("\nRunning new Webbox");

    let args: Vec<String> = env::args().collect();
    let config_dir = get_config_dir();
    let config_file = read_config(&config_dir.join("config.webbox"));

    match args.len() {
        1 => {
            // argument message
            println!("No commands passed, running default process\n");
            // loop through the links
            for link in config_file.lines() { open_link(link, 50) }
            // empty line for aestetics
            println!("");
            // macOS hack to prevent hanging terminal 
            macos_exit();
        },
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "-help" => process_help(cmd),
                "-list" => process_list(cmd, config_dir),
                "-config-path" => println!("{} argument passed, running {} process\n\n\"config.webbox\" file location : {}\n", cmd, cmd, config_dir.display()),
                "-new-link" => missing_args(cmd),
                "-remove-link" => missing_args(cmd),
                "-update-link" => missing_args(cmd),
                "-swap-link" => missing_args(cmd),
                _ => eprintln!("Error: Unknown process \"{}\"\nUse -help to get a list of available commands\n", cmd)
            }
        },
        3 => {
            let cmd = &args[1];
            let arg = &args[2];
            match &cmd[..] {
                "-help" => extra_args(cmd, arg),
                "-list" => extra_args(cmd, arg),
                "-config-path" => extra_args(cmd, arg),
                "-new-link" => process_new(cmd, arg, config_dir),
                "-remove-link" => process_remove(cmd, arg, config_dir),
                "-update-link" => missing_args(cmd),
                "-swap-link" => missing_args(cmd),
                _ => eprintln!("Error: Unknown process \"{}\"\nUse -help to get a list of available commands\n", cmd)
            }
        },
        4 => {
            let cmd = &args[1];
            let arg0 = &args[2];
            let arg1 = &args[3];
            match &cmd[..] {
                "-help" => extra_args(cmd, arg1),
                "-list" => extra_args(cmd, arg1),
                "-config-path" => extra_args(cmd, arg1),
                "-new-link" => extra_args(cmd, arg1),
                "-remove-link" => extra_args(cmd, arg1),
                "-update-link" => process_update(cmd, arg0, arg1, config_dir),
                "-swap-link" => process_swap(cmd, arg0, arg1, config_dir),
                _ => eprintln!("Error: Unknown process \"{}\"\nUse -help to get a list of available commands\n", cmd)
            }
        }
        _ => eprintln!("Error: Unknown or too many arguments\nUse -help to get a list of available commands\n")
    }
}

fn process_help(cmd: &String) {
    // help sausage
    println!("{} command passed, running {} process\n\nRunning Webbox without commands will activate the main process (opening the tabs)\n\nHere are the available commands for Webbox!:\n-help : list of arguments | eg : ./webbox -help\n-list : list the existing links | eg : ./webbox -list\n-config-path : config file location | eg : ./webbox -config-path\n-new-link [link] : add link to list | eg : ./webbox -new-link https://www.example.com\n-update-link [index] [link] : update existing link | eg : ./webbox -update-link 2 https://www.example.com\n-remove-link [index] : remove existing link | eg : ./webbox -remove-link 2\n-swap-link [index] [index] : swap link position with another link | eg: -swap-link 2 1\n", cmd, cmd);
}

fn process_list(cmd: &String, config_dir: PathBuf) {
    process_message(cmd);
    let config_file = read_config(&config_dir.join("config.webbox"));
    // enumerate through the links
    for (index, link) in config_file.lines().enumerate() { println!("{} : {}", index, link) }
    // empty line for aestetics
    println!("");
}

fn process_new(cmd: &String, arg: &String, config_dir: PathBuf) {
    process_message(cmd);
    let config_file = read_config(&config_dir.join("config.webbox"));
    // add the new link to the string
    let update_content = config_file + arg;
    // update the file
    update_config(&config_dir.join("config.webbox"), &update_content)
}

fn process_remove(cmd: &String, arg: &String, config_dir: PathBuf) {
    process_message(cmd);
    let config_file = read_config(&config_dir.join("config.webbox"));
    // get link index
    let index: usize = arg.parse().expect("Error: Failed to parse [index], no a possitive integer\n");
    // initialize the list
    let mut list =  vec![];
    // push each link inside the list
    for link in config_file.lines() { list.push(link) }
    // is the index in range
    if index < list.len() {
        // remove the link at index
        list.remove(index);
        // join the links back to a string
        let update_content = list.join("\n");
        // update the file
        update_config(&config_dir.join("config.webbox"), &update_content);
    }
    else { eprintln!("Error: index = {} is not inside the list\nUse -list to find all the links and available indexes\n", arg) }
}

fn process_update(cmd: &String, arg_index: &String, arg_link: &String, config_dir: PathBuf) {
    process_message(cmd);
    let config_file = read_config(&config_dir.join("config.webbox"));
    // get link index
    let index: usize = arg_index.parse().expect("Error: Failed to parse [index], no a possitive integer\n");
    // initialize the list
    let mut list =  vec![];
    // push each link inside the list
    for link in config_file.lines() { list.push(link) }
    // is the index in range
    if index < list.len() {
        // update the link at index
        list[index] = arg_link;
        // join the links back to a string
        let update_content = list.join("\n");
        // update the file
        update_config(&config_dir.join("config.webbox"), &update_content);
    }
    else { eprintln!("Error: index = {} is not inside the list\nUse -list to find all the links and available indexes\n", index) }
}

fn process_swap(cmd: &String, arg_old: &String, arg_new: &String, config_dir: PathBuf) {
    process_message(cmd);
    let config_file = read_config(&config_dir.join("config.webbox"));
    // get link index
    let index_old: usize = arg_old.parse().expect("Error: Failed to parse [index], no a possitive integer\n");
    let index_new: usize = arg_new.parse().expect("Error: Failed to parse [index], no a possitive integer\n");
    // initialize the list
    let mut list =  vec![];
    // push each link inside the list
    for link in config_file.lines() { list.push(link) }
    // is the index in range
    if index_old < list.len() && index_new < list.len() {
        // temp hold index link
        let temp = list[index_old];
        // update the link at index with link at index2
        list[index_old] = list[index_new];
        // update the link at index 2 with the link at index
        list[index_new] = temp;
        // join the links back to a string
        let update_content = list.join("\n");
        // update the file
        update_config(&config_dir.join("config.webbox"), &update_content);
    }
    else { eprintln!("Error: index_0 = {} or index_1 = {} are not inside the list\nUse -list to find all the links and available indexes\n", index_old, index_new) }
}

fn process_message(cmd: &String) {
    // argument message
    println!("{} argument passed, running {} process\n", cmd, cmd);
}

fn open_link(link: &str, ms: u64) {
    // if the link can open print success
    if webbrowser::open(link).is_ok() { println!("Success: Opened the link {}", link) }
    // if the link can not open print error
    else { eprintln!("\nError: Failed to open the link {}", link) }
    // sleep to prevent pages from opening in an unordered manner
    sleep(Duration::from_millis(ms));
}

fn read_config(file_path: &Path) -> String {
    if !file_path.exists() { update_config(file_path, "") }
    return fs::read_to_string(file_path).expect("\nError: Failed to read the \"config.webbox\" file");
}

fn update_config(file_path: &Path, update_content: &str) {
    let content = if file_path.exists() { update_content }
        else { "https://github.com/ionutrogojan/webbox\nhttps://www.rust-lang.org/\nhttps://dictionary.cambridge.org/us/dictionary/english/blazingly" };
        
    let mut config_file = match File::create(&file_path) {
        Ok(file) => file,
        Err(e) => panic!("\nError creating file {} :\n {}\n", file_path.display(), e)
    };
    // write file
    match config_file.write_all(content.as_bytes()) {
        Ok(_) => println!("\"config.webbox\" file created with default values\n"),
        Err(e) => panic!("\nError writing file {} :\n {}\n", file_path.display(), e)
    };
}

fn get_config_dir() -> PathBuf {
    // os path + webbox directory
    let config_dir = if cfg!(target_os = "windows") { dirs::preference_dir().expect("\nError: Unable to access \"AppData\" directory").join("Webbox") }
        else { dirs::home_dir().expect("\nError: Unable to access \"$HOME\" directory").join(".webbox") };

    if !config_dir.exists() {
        // create directory
        match fs::create_dir(config_dir.clone()) {
            Ok(_) => println!("Created \"Webbox\" directory at {}", config_dir.display()),
            Err(e) => panic!("\nError creating directory {}:\n{}\n", config_dir.display(), e)
        }
    }
    return config_dir; 
}

fn macos_exit() {
    // spawn "kill" process to close terminal after execution on macos
    if cfg!(target_os = "macos") { Command::new("kill").arg("Terminal").spawn().expect("\nError: Failed to close Terminal application"); }
    process::exit(0);
}

fn extra_args(cmd: &String, arg: &String) {
    eprintln!("{} argument passed, running {} process\n\nError: \"{}\" has too many arguments \"{}\"\nUse -help to get a list of available commands\n", cmd, cmd, cmd, arg)
}

fn missing_args(cmd: &String) {
    eprintln!("{} argument passed, running {} process\n\nError: \"{}\" takes more arguments\nUse -help to get a list of available commands\n", cmd, cmd, cmd)
}
