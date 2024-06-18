// drogi programisto
// tylko ja i Bóg wiedzieliśmy jak działa ten kod
// teraz tylko Bóg wie
// jebać debiana arch najlepszy
#[allow(unused)]
use std::process::{Command, exit};
use std::{fs, process, thread, vec};
use std::arch::asm;
use std::env::{args, current_dir, current_exe};
use std::fs::{create_dir, read_dir, rename};
use std::io::{BufRead, Read, read_to_string, stdout, Write};
use std::iter::StepBy;
use std::path::{Path, PathBuf};
use std::thread::current;
use std::time::{Duration, SystemTime};
extern crate chrono;
use chrono::prelude::*;
use sysinfo::{Cpu, System};
use sysinfo::Signal::Sys;
extern crate whoami;
use whoami::{fallible, platform};
extern crate system_shutdown;
extern crate clear_screen;

fn main(){
    let mut terminal_history = vec![];
    loop {
        print!(">");
        let _ = stdout().flush();
        let mut quest = String::new();
        std::io::stdin().read_line(&mut quest).expect("an error occurred while reading input");
        terminal_history.push(quest.clone());

        match quest.trim() {
            "mdir" | "mkdir" | "ndir" | "newdir" =>{
                println!("enter name of the folder");
                print!(">");
                let _ = stdout().flush();
                let mut args = String::new();
                std::io::stdin().read_line(&mut args).expect("an error occurred while reading input");

                println!("enter number of folders"); // to dla Pana Pietrzaka
                print!(">");
                let _ = stdout().flush();

                let mut quantity = String::new();
                std::io::stdin().read_line(&mut quantity).expect("an error occurred while reading user input");
                let quantity: i32 = quantity.trim().parse().expect("an error occurred while converting user input");

                if quest.trim() != ""{
                    if quantity == 1{
                        fs::create_dir_all(args.trim()).expect("error while creating directory");
                    }
                    else {
                        for i in 1..quantity + 1 {
                            let path: String = args.trim().to_owned() + &i.to_string();
                            fs::create_dir_all(path).expect("error while creating directory");
                        }
                    }
                }
                else {
                    println!("error! you can't leave this empty!");
                }
            },
            "nfile" | "touch" | "newfile" =>{
                println!("enter name of the file");
                print!(">");
                let _ = stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).expect("an error occurred while reading input");

                println!("enter extension of the file e.g. txt or exe");
                print!(">");
                let _ = stdout().flush();

                let mut extension = String::new();
                std::io::stdin().read_line(&mut extension).expect("an error occurred while reading input");

                println!("enter number of files");
                print!(">");
                let _ = stdout().flush();

                let mut quantity = String::new();
                std::io::stdin().read_line(&mut quantity).expect("an error occurred while taking user input");
                let quantity: i32 = quantity.trim().parse().expect("an error occurred while converting user input");

                if args.trim() != "" {
                    if quantity == 1 {
                        let path = args.trim().to_owned() + stringify!(.) + extension.trim();
                        fs::File::create_new(path).expect("error while creating file");
                    }
                    else {
                        for i in 1..quantity + 1 {
                            let path = args.trim().to_owned() + &i.to_string() + stringify!(.) + extension.trim();
                            fs::File::create_new(path).expect("error while creating file");
                        }
                    }
                }
                else {
                    println!("error! you can't leave name empty!")
                }
            },
            "rm" | "remove" | "rmfile" | "rfile" =>{
                println!("which file do you want to remove?");

                print!(">");
                let _ = stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).expect("error while reading user input");

                if args.trim() != "" {
                    fs::remove_file(args.trim()).expect("error while removing file");
                }
                else {
                    println!("invalid option! you can't leave this empty!");
                }
            },
            "rdir" | "rmdir" | "removedir" | "removedirectory" =>{
                println!("enter directory you want to delete (with all of containing files)");
                print!(">");
                let _ = stdout().flush();
                let mut args = String::new();
                std::io::stdin().read_line(&mut args).expect("error while reading user input");

                if args.trim() == "/" || args.trim() == "*" || args.trim() == r#"C:\"# || args.trim() == r#"C:\\"# { // dreams of Lars and a baked apple pie
                    println!("are you sure you want to do this? executing this command will erase all data on disk yes/no ");
                    print!(">");
                    let _ = stdout().flush();

                    let mut quest = String::new();
                    std::io::stdin().read_line(&mut quest).expect("error while reading user input");
                    let quest = quest.trim().to_uppercase();

                    if quest == "YES" {
                        fs::remove_dir_all(args.trim()).expect("an error occurred while deleting system partition (thank God)"); //usuwa / albo C:\ więc lepiej wara od tego
                    }
                    else {
                        println!("aborting thread");
                    }
                }
                else if !(args.trim() == "") {
                    fs::remove_dir_all(args.trim()).expect("error while deleting directory");
                }
                else {
                    println!("error! you can't leave this blank")
                }
            },
            "cd" | "goto" =>{
                println!("enter target directory");
                print!(">");
                let _ = stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).expect("error while reading user input");

                if !(args.trim() == ""){
                    std::env::set_current_dir(args.trim()).expect("an error occurred during changing working directory");
                }
                else {
                    println!("error! you can't leave this blank!");
                }
            },
            "pwd" | "here" | "whereami" =>{
              let current_dir = current_dir();
              println!("working direcory is {} ", current_dir.expect("error - cannot get working directory").display());
            },
            "list" | "ls" | "listfiles" | "lsdir" | "listdirectory" =>{
                println!("which directory do you want to list? (leave blank for current)");
                print!(">");
                let _ = stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).expect("an error occurred while reading input");

                if args.trim() == "" {
                    let path = read_dir(current_dir().expect("an error occurred while getting current directory")).expect("error - cannot list current directory");
                    for files in path{
                        println!("Name: {}", files.unwrap().path().display());
                    }
                }
                else {
                    let path = read_dir(args.trim()).expect("error - cannot read this directory");
                    for files in path{
                      println!("name: {}", files.expect("an error occurred while trying to read this file").path().display());
                  }
                }
            },
            "fread" | "read"=>{
                println!("enter file name");
                print!(">");
                let _ = stdout().flush();

                let mut path = String::new();
                std::io::stdin().read_line(&mut path).expect("error during taking input from user");
                let path = path.trim();

                match fs::read_to_string(path){
                    Ok(contents) => println!("file {} contains:\n{}", path, contents),
                    Err(e) => println!("error reading file {}: {}", path, e),
                }
            },
            "rname" | "rename" =>{
                println!("enter filename");
                print!(">");
                let _ = stdout().flush();

                let mut args1 = String::new();
                std::io::stdin().read_line(&mut args1).expect("an error occurred while taking user input");
                let args1 = args1.trim();

                println!("enter target filename");
                print!(">");
                let _ = stdout().flush();

                let mut args2 = String::new();
                std::io::stdin().read_line(&mut args2).expect("error while reading user input");
                let args2 = args2.trim();

                rename(args1, args2).expect("an error occurred while renaming file");
            },
            "whoami" | "info" | "who" =>{
                clear_screen::clear();
                println!("here is your hardware and software info:");
                thread::sleep(Duration::from_millis(150));
                println!("username: {}",whoami::username());
                thread::sleep(Duration::from_millis(50));
                println!("user's real name: {}", whoami::realname()); // jak chinole szpiegujemy ludzi
                thread::sleep(Duration::from_millis(50));
                // println!("language: {:?}",whoami::langs().collect::<Vec<String>>()); //ogl jebać vektory
                // thread::sleep(Duration::from_millis(50));
                println!("device's name: {}",whoami::devicename());
                thread::sleep(Duration::from_millis(50));
                match fallible::hostname(){
                    Ok(hostname) => println!("device's hostname {}",hostname),
                    Err(error) => println!("error while reading hostname: {}",error)
                }
                thread::sleep(Duration::from_millis(50));
                println!("device's platform: {}",whoami::platform());
                thread::sleep(Duration::from_millis(50));
                println!("device's distro: {}",whoami::distro());
                thread::sleep(Duration::from_millis(50));
                match System::kernel_version(){
                    Some(kernel_version) => println!("kernel version: {}",kernel_version), // mmm kernel, mój ulubieny
                    _ => println!("error occurred while reading kernel version"),
                }
                thread::sleep(Duration::from_millis(50));
                match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){ // unix najlepszy i kompatybilny ze wszystkim
                    Ok(time) => println!("system time since UNIX epoch: {:?}",time),
                    _ => println!("error - cannot read time"),
                }
                thread::sleep(Duration::from_millis(50));
                println!("device's desktop environment: {}",whoami::desktop_env());
                thread::sleep(Duration::from_millis(50));
                println!("CPU architecture: {}",whoami::arch());
                thread::sleep(Duration::from_millis(50));
                let disks = sysinfo::Disks::new_with_refreshed_list();
                println!("data storages:");
                let mut count: i8 = 0;
                for disk in &disks{
                    count+=1;
                    println!(" {}.volume: {:?}",count,disk);
                }
            }
            "find" | "istherea" =>{
                println!("enter name of the file you want to check");
                print!(">");
                let _ = stdout().flush();

                let mut filename = String::new();
                std::io::stdin().read_line(&mut filename).expect("an error occurred while taking user input");
                let filename = filename.trim(); // skurwiel zmarnował mi kilkanaście minut życia

                println!("enter phrase you want to search for");
                print!(">");
                let _ = stdout().flush();

                let mut phrase = String::new();
                std::io::stdin().read_line(&mut phrase).expect("an error occurred while taking user input");

                match fs::read_to_string(filename){
                    Ok(contains) =>{
                        if contains.contains(&phrase) { // czasem działa czasem nie, a dlaczego chuj to wie
                            println!("yes, {:?} contains {:?}", filename,phrase.trim());
                        }
                        else {
                            println!("no, {:?} does not contain {:?}",filename,phrase.trim());
                        }
                    },
                    Err(error) => println!("an error occurred while reading file {:?}, error message: {}",filename.trim(), error), // os error 123 no japierdole
                }
            },
            "move" | "relocate" | "mv" =>{
                println!("enter file that you want to relocate");
                print!(">");
                let _ = stdout().flush();

                let mut current = String::new();
                std::io::stdin().read_line(&mut current).expect("an error occurred during input reading");
                let current = current.trim();

                println!("enter destination");
                print!(">");
                let _ = stdout().flush();

                let mut destination = String::new();
                std::io::stdin().read_line(&mut destination).expect("error - cannot read user input");
                let destination = destination.trim();

                match rename(current, destination){
                    Ok(_) => println!("file moved successfully"),
                    Err(error_code) => println!("and error occurred while relocating file: {}", error_code), // odmowa dostępu do mózgu kurwa
                } // a nie, teraz error 5 a nie 123
            },
            "time" | "clock" =>{
                let current_time: DateTime<Local> = Local::now(); // chuj wie jak działa ale działa więc nie dotykać tego
                println!("current time is {:?}", current_time);
            },
            "history" =>{
                println!("your terminal history contains: ");
                let mut history_count: i128 = 0;
                for i in &terminal_history{ //jednak kocham vektory i jebac array
                    history_count += 1;
                    println!("\t{}.{}",history_count,i.trim());
                }
                println!("if you want to clear history, type 'history clear'");
            }
            "history clear" =>{ // mama i tak się dowie
                terminal_history.clear();
            }
            "leave" | "exit" | "close" =>{
                exit(0);
                // unsafe { // jebać windowsa bo syscallsy są do dupy
                //     asm!(
                //         "syscall",
                //         in("rax") 60,
                //         in("rdi") 0,
                //         out("rcx") _,
                //         out("r11") _,
                //     )
                // }
            },
            "shutdown" | "turnoff" =>{
                match system_shutdown::shutdown(){
                    Ok(_) => println!("shutting down!"),
                    Err(error_code) => {
                        println!("cannot shut machine down due to error: {}",error_code);
                        thread::sleep(Duration::from_millis(150));
                        println!("you can force shutdown using 'force shutdown'");
                    },
                }
            },
            "force shutdown" =>{ // of course not!
                match system_shutdown::force_shutdown(){
                    Ok(_) => println!("shutting down!"),
                    Err(error_code) => println!("an error occurred while forcing shutdown: {}",error_code),
                }
            }
            "clear" | "cls" =>{
                clear_screen::clear();
            },
            "help" =>{
              println!(" leave/exit/close - exit terminal\n clear/cls - clear screen\n mkdir/mdir/ndir/newdir - creates new empty directory\n nfile/newfile/touch - make file\n rm/remove/rmfile/rfile - remove single file\n rdir/rmdir/removedir/removedirectory - removes directory with all containing files\n cd/goto - change directory\n pwd/whereami/here - show working directory\n list/ls/listfiles/lsdir/listdirectory - shows files in directory\n time/clock - displays current time\n fread/read - prints file contains\n rename/rname - renames file/folder\n whoami/info/who - displays info about hardware and software\n history/history clear - shows/clears terminal history\n find/istherea - searches given phrase in specified file\n move/relocate/mv - moves file to another destination\n shutdown/turnoff - turns of your machine");
            },
            _ => {
                println!("invalid option! type 'help' for full command list");
            }
        }
    }
}

// fn clear_screen(){
//     if cfg!(target_os = "windows"){
//         Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
//     }
//     else{
//         Command::new("clear").status().unwrap();
//     }
// }