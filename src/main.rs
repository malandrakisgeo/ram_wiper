use std::alloc::{alloc_zeroed, Layout};
use std::{env, process};
use nix::unistd::Pid;
use nix::sys::signal::{self, Signal};
use regex::Regex;


fn main() {
    //a flag -k will find and kill all user threads, thereby deallocating their ram for overwriting.
    let args: Vec<String> = env::args().collect();

    if(args.len() > 1){
        if &args[1].eq("-k") {
            process_killer();
        }
    }

     unsafe {
         loop {
              alloc_zeroed(Layout::from_size_align(2048, 4).unwrap());
          }
      };
}

fn process_killer() {
    let ancestral_pids =  get_all_parents(process::id());

    let usr_procs = user_processes();
    let pid: Vec<&u32> = usr_procs
        .iter()
        .filter(|pid| !ancestral_pids.contains(pid))
        .rev()
        .collect();

    for x in pid {
        signal::kill(Pid::from_raw(*x as  i32), Signal::SIGTERM).unwrap_or_else(|e| {
            println!("Could not terminate: {}", e);
        });
    }

    return;
}

//Retrieves all processes of the current user
fn user_processes() -> Vec<u32> {
    let mut pids: Vec<u32> = Vec::new();
    let procs = process::Command::new("ps")
        .arg("-u")
        .arg(format!("{}", env::var("USER").unwrap()))
        .arg("-o")
        .arg("pid")
        .output();

    let output = String::from_utf8_lossy(&procs.unwrap().stdout).to_string();

    for pid in output.split("\n") {
        match pid.trim().parse::<u32>() {
            Ok(p) => pids.push(p),
            Err(_) => ()
        }
    }

    return pids;
}


/*
get_all_parents retrieves the pids of the processes that are ancestral to ram_wiper.
We need them in order to make an exception for them in process_killer
e.g.:
pstree -s -p 41319
    systemd(1)───systemd(36898)───gnome-shell(37174)───konsole(41305)───bash(41319)───pstree(42030)
 */
fn get_all_parents(id: u32)  -> Vec<u32>{
    let mut parent_pids: Vec<u32> = Vec::new();
    let parent_procs = process::Command::new("pstree")
        .arg("-s")
        .arg("-p")
        .arg(format!("{}", id))
        .output();
    let output2 = String::from_utf8_lossy(&parent_procs.unwrap().stdout).to_string();

    let re: Regex = Regex::new("\\(([^\\)]+)\\)").unwrap();
    println!("{}", output2);

    for x in re.find_iter(&*output2){
        let aa = x.as_str().replace("(", "").replace(")", "");
        match aa.trim().parse::<u32>() {
            Ok(p) => parent_pids.push(p),
            Err(_) => ()
        }
    }

    return parent_pids;

}



