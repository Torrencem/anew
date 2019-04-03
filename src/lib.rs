use std::env;
use std::process::exit;

use std::path::{Path, PathBuf};
use std::os;
use std::fs;
use std::io;

fn get_template_dir() -> io::Result<PathBuf> {
    let key = "HOME";
    let home_directory = env::var(key).unwrap_or_else(|e| {
        eprintln!("Could not find or decode user home directory");
        eprintln!("Try re-running with $HOME set\n");
        eprintln!("{}", e);
        exit(1);
    });

    let home = Path::new(&home_directory);
    let template_dir = home.join(".anew").join("templates");

    if !template_dir.exists() {
        match fs::create_dir_all(&template_dir) {
            Err(x) => {
                eprintln!("There was a problem creating a $HOME/.anew directory\n");
                return Err(x);
            },
            _ => ()
        }
    }

    if !template_dir.is_dir() {
        eprintln!("Template directory in .anew/templates is not a directory");
        fs::create_dir_all(&template_dir)?;
    }

    Ok(template_dir)
}

fn greatest_common_ancestor<P: AsRef<Path> + Clone>(paths: &Vec<P>) -> Option<PathBuf> {
    // Choose an arbitrary path in our list,
    // and find a common ancestor
    paths[0].as_ref().ancestors().find(|ancestor| {
        paths.iter().all(|path| {
            path.as_ref().starts_with(ancestor)
        })
    }).map(|path| {
        path.to_path_buf()
    })
}

pub fn list_templates() -> io::Result<()> {
    let tdir = get_template_dir()?;
    println!("* All Templates:");
    
    for entry in fs::read_dir(tdir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            // Maybe get more information from metadata here?
            // i.e. last modified date, etc.
            println!("  -{}", name.into_string().unwrap_or_else(|_| {
                eprintln!("Fatal: Invalid unicode data in directory name");
                eprintln!("This is an uncommon problem. Try removing offending");
                eprintln!("directory from .anew/templates/");
                exit(1);
            }));
        }
    }

    Ok(())
}

pub fn create_template(name: &String, files: Vec<PathBuf>, link: bool) -> io::Result<()> {
    let current_path = env::current_dir()?;
    // First try stripping paths to the current directory
    let un_prefixed_res: Vec<Result<&Path, _>> = files.iter().map(|path| {
        path.strip_prefix(current_path.clone())
    }).collect();
    let un_prefixed: Vec<PathBuf>;
    if un_prefixed_res.iter().all(|p| p.is_ok()) {
        un_prefixed = un_prefixed_res.iter().map(|path| {
            path.clone().unwrap().to_path_buf()
        }).collect();
    } else {
        // If that fails, try stripping paths to their greatest
        // common ancestor. TODO: Maybe add a flatten option to the CLI
        let gca = greatest_common_ancestor(&files).unwrap();
        un_prefixed = files.iter().map(|path| {
            path.strip_prefix(&gca).unwrap().to_path_buf()
        }).collect();
    }

    // Copy the files from their old place to their template copy
    let mut tdir = get_template_dir()?;
    tdir.push(name);

    for (full_path, relative_path) in files.iter().zip(un_prefixed.iter()) {
        let mut new_path = PathBuf::new();
        new_path.push(tdir.clone());
        new_path.push(relative_path);
        // Make sure it's parent folder exists
        if let Some(path) = new_path.parent() {
            fs::create_dir_all(path)?;
        }
        if link {
            // Symlink files/directories
            // Depends on os (https://doc.rust-lang.org/std/fs/fn.soft_link.html)
            #[cfg(target_os = "windows")] {
                if full_path.is_dir() {
                    os::windows::fs::symlink_dir(full_path, new_path)?;
                } else {
                    os::windows::fs::symlink_file(full_path, new_path)?;
                }
            }
            #[cfg(target_os = "linux")] {
                os::unix::fs::symlink(full_path, new_path)?;
            }
        } else {
            // Copy file/directory into new location
            if full_path.is_dir() {
                fs::create_dir_all(new_path)?;
            } else {
                fs::copy(full_path, new_path)?;
            }
        }
    }
    Ok(())
}

pub fn remove_template(name: &String) -> io::Result<()> {
    let mut tdir = get_template_dir()?;
    tdir.push(name);
    if !tdir.exists() {
        eprintln!("Template does not exist: {}", name);
        // Probably ok, template is deleted
        return Ok(());
    }
    // NOTE: remove_dir_all does NOT follow symbolic (soft) links
    // So it will not delete the original files if they were linked
    fs::remove_dir_all(tdir)?;
    Ok(())
}

pub fn apply_template(name: &String, path: PathBuf) -> io::Result<()> {
    let mut tdir = get_template_dir()?;
    tdir.push(name);
    if !tdir.exists() {
        eprintln!("Template does not exist: {}", name);
        eprintln!("(use 'anew dir' to list available templates)");
        exit(1);
    }
    cp_from_dir(tdir.as_path(), &path, &tdir)?;
    return Ok(());
}

// based on example from https://doc.rust-lang.org/std/fs/fn.read_dir.html
fn cp_from_dir(dir: &Path, destination: &PathBuf, tdir: &PathBuf) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(tdir).unwrap();
            let path = entry.path().canonicalize().unwrap();
            if path.is_dir() {
                let mut new_dir_path = PathBuf::new();
                new_dir_path.push(destination);
                new_dir_path.push(relative_path);
                fs::create_dir_all(new_dir_path)?;
                cp_from_dir(&path, destination, tdir)?;
            } else {
                // There is a file we need to copy
                let mut new_file_path = PathBuf::new();
                new_file_path.push(destination);
                new_file_path.push(relative_path);
                fs::copy(path, new_file_path)?;
            }
        }
    }
    Ok(())
}