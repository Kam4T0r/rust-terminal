use std::process::{Command, exit};
use std::{fs, process, thread};
use std::env::current_dir;
use std::fs::{create_dir, read_dir};
use std::io::{Read, stdout, Write};
use std::iter::StepBy;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn main(){
    loop {
        print!(">");
        stdout().flush();
        let mut quest = String::new();
        std::io::stdin().read_line(&mut quest).unwrap();

        match quest.trim() {
            "mdir" | "mkdir" | "ndir" | "newdir" =>{
                println!("enter name of the folder");
                print!(">");
                stdout().flush();
                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                println!("enter number of folders");
                print!(">");
                stdout().flush();

                let mut quantity = String::new();
                std::io::stdin().read_line(&mut quantity).expect("error");
                let mut quantity: i32 = quantity.trim().parse().expect("error");

                if quest.trim() != ""{
                    if quantity == 1{
                        fs::create_dir_all(args.trim()).unwrap();
                    }
                    else {
                        for i in 1..quantity + 1 {
                            let mut path: String = args.trim().to_owned() + &i.to_string();
                            fs::create_dir_all(path).unwrap();
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
                stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                println!("enter extension of the file e.g. txt or exe");
                print!(">");
                stdout().flush();

                let mut extension = String::new();
                std::io::stdin().read_line(&mut extension).unwrap();

                println!("enter number of files");
                print!(">");
                stdout().flush();

                let mut quantity = String::new();
                std::io::stdin().read_line(&mut quantity).expect("error");
                let mut quantity: i32 = quantity.trim().parse().expect("error");

                if args.trim() != "" {
                    if quantity == 1 {
                        let mut path = args.trim().to_owned() + stringify!(.) + extension.trim();
                        fs::File::create_new(path).unwrap();
                    }
                    else {
                        for i in 1..quantity + 1 {
                            let mut path = args.trim().to_owned() + &i.to_string() + stringify!(.) + extension.trim();
                            fs::File::create_new(path).unwrap();
                        }
                    }
                }
                else {
                    println!("error! you can't leave name empty!")
                }
            },
            "rm" | "remove" | "rmfile" | "rfile" =>{
                println!("which file do you want to remove? (with extension)");

                print!(">");
                stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                if args.trim() != "" {
                    fs::remove_file(args.trim()).unwrap();
                }
                else {
                    println!("invalid option! you can't leave this empty!");
                }
            },
            "rdir" | "rmdir" | "removedir" | "removedirectory" =>{
                println!("enter directory you want to delete (with all of containing files)");
                print!(">");
                stdout().flush();
                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                if args.trim() == "/" || args.trim() == "*" {
                    println!("are you sure you want to do this? executing this command will erase all data on disk yes/no ");
                    print!(">");
                    stdout().flush();

                    let mut quest = String::new();
                    std::io::stdin().read_line(&mut quest).unwrap();
                    let mut quest = quest.trim().to_uppercase();

                    if quest == "YES" {
                        fs::remove_dir_all(args.trim()).unwrap();
                    }
                    else {
                        println!("aborting thread");
                    }
                }
                else if !(args.trim() == "") {
                    fs::remove_dir_all(args.trim()).unwrap();
                }
                else {
                    println!("error! you can't leave this blank")
                }
            },
            "cd" | "goto" =>{
                println!("enter target directory");
                print!(">");
                stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                if !(args.trim() == ""){
                    std::env::set_current_dir(args.trim()).unwrap();
                }
                else {
                    println!("error! you can't leave this blank!");
                }
            },
            "pwd" | "here" | "whereami" =>{
              let mut current_dir = std::env::current_dir();
              println!("working direcory is {} ", current_dir.unwrap().display());
            },
            "list" | "ls" | "listfiles" | "lsdir" | "listdirectory" =>{
                println!("which directory do you want to list? (leave blank for current)");
                print!(">");
                stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                if args.trim() == "" {
                    let mut path = fs::read_dir(std::env::current_dir().unwrap()).unwrap();
                    for files in path{
                        println!("Name: {}", files.unwrap().path().display());
                    }
                }
                else {
                    let mut path = fs::read_dir(args.trim()).unwrap();
                    for files in path{
                      println!("name: {}", files.unwrap().path().display());
                  }
                }
            },
            "leave" | "exit" | "close" =>{
                process::exit(0);
            },
            "clear" | "cls" =>{
                clear_screen();
            },
            "help" =>{
              println!(" leave/exit/close - exit terminal\n clear/cls - clear screen\n mkdir/mdir/ndir/newdir - creates new empty directory\n nfile/newfile/touch - make file\n rm/remove/rmfile/rfile - remove single file\n rdir/rmdir/removedir/removedirectory - removes directory with all containing files\n cd/goto - change directory\n pwd/whereami/here - show working directory\n list/ls/listfiles/lsdir/listdirectory - shows files in directory");
            },
            _ => {
                println!("invalid option! type 'help' for full command list");
            }
        }
    }
}

fn clear_screen(){
    if cfg!(target_os = "windows"){
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    }
    else{
        Command::new("clear").status().unwrap();
    }
}