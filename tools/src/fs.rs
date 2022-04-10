use std::{fs::{File, OpenOptions, read_dir}, sync::{Arc, Mutex}, io::{Read, SeekFrom, Seek, Write}};
use snail_fs::{BlockDev, SnailFileSystem};

const BLOCK_SZ: usize = 512;

struct BlockFile(Mutex<File>);

impl BlockDev for BlockFile {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        let mut file = self.0.lock().unwrap();
        file.seek(SeekFrom::Start((block_id * BLOCK_SZ) as u64))
            .expect("[Error] something is wrong when seeking");
        assert_eq!(file.read(buf).unwrap(), BLOCK_SZ, "[Error] Not a complete block!");
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        let mut file = self.0.lock().unwrap();
        file.seek(SeekFrom::Start((block_id * BLOCK_SZ) as u64))
            .expect("[Error] something is wrong when seeking");
        assert_eq!(file.write(buf).unwrap(), BLOCK_SZ, "[Error] Not a complete block!");
    }
}

pub fn pack_all_apps() -> std::io::Result<()> {
    use clap::{Arg, Command};
    let matches = Command::new("SnailFileSystem packer")
        .arg(Arg::new("source")
            .short('s')
            .long("source")
            .takes_value(true)
            .help("Executable source dir(with backslash)")
        )
        .arg(Arg::new("target")
            .short('t')
            .long("target")
            .takes_value(true)
            .help("Executable target dir(with backslash)")
        )
        .get_matches();
    let src_path = matches.value_of("source").unwrap();
    let target_path = matches.value_of("target").unwrap();
    println!("src_path = {}\ntarget_path = {}", src_path, target_path);
    let block_file = Arc::new(
        BlockFile(
            Mutex::new({
                let f = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(format!("{}{}", target_path, "fs.img"))?;
                f.set_len(8192 * 512).unwrap();
                f
            })
        )
    );
    // 4MB, at most 4095 files
    let sfs = SnailFileSystem::create(
        block_file.clone(), 8192, 1
    );
    let root_inode = Arc::new(SnailFileSystem::root_inode(&sfs));
    let apps: Vec<_> = read_dir(src_path)
        .unwrap()
        .into_iter()
        .map(|dirent| {
            let mut name_with_ext = dirent.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    for app in apps {
        // load app data from host file system
        let mut host_file = File::open(format!("{}{}", target_path, app)).unwrap();
        let mut all_data: Vec<u8> = Vec::new();
        host_file.read_to_end(&mut all_data).unwrap();
        // create a file in snail fs
        let inode = root_inode.create(app.as_str()).unwrap();
        // write data to snail fs
        inode.write_at(0, all_data.as_slice());
    }
    for app in root_inode.ls() {
        println!("{}", app);
    }

    Ok(())
}