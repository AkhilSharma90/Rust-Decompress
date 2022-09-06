use std::fs;
use std::io;

fn main() {
    
    //process the real program in the real_main function
    //for clean end and exit use this in the main function
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    //create a vector called args to collect users input in the CLI
    let args: Vec<_> = std::env::args().collect();
    //if args less than 2, there's an issue because  you need to send the name of the 
    // zip file and it'll show you how to use
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }
    //args at the 2nd position, denoted by 1st index is the file name 
    let fname = std::path::Path::new(&*args[1]);
    //open the file using standard fs
    let file = fs::File::open(&fname).unwrap();

    //using the archive reader function
    let mut archive = zip::ZipArchive::new(file).unwrap();

//start from 0 and cover the entire length of archive
// there will be multiple files in the zip archive and we need to extract all
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
    //setting the path where the files will be extracted
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
//the zip can contain other folders too
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
    //recursively create a new directory
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            //if there is no parent for those files, create a new directory
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions for the extracted files
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}