use std::io::{Stdout, Stdin, self, Write};
use std::process::Command;
use std::time::Duration;
use sha256::digest;
use std::fs::{File, self, OpenOptions, remove_file};
use std::io::Read;
use std::path::Path;
use dll_syringe::{Syringe, process::OwnedProcess};

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    let mut stdout: Stdout = io::stdout();
    let stdin: Stdin = io::stdin();

    println!("Welcome to the (Scuffed) ReBorn launcher!");

    if(locate_binary_and_patch()){
        println!("Type the number of the PvE mission you want to play:");
        
        println!("1) Prologue - Story Mission 0");
        println!("2) The Algorithm - Story Mission 1");
        println!("3) Void's Edge - Story Mission 2");
        println!("4) The Renegade - Story Mission 3");
        println!("5) The Archive - Story Mission 4");
        println!("6) Sentinel - Story Mission 5");
        println!("7) The Experiment - Story Mission 6");
        println!("8) The Saboteur - Story Mission 7");
        println!("9) Heliophage - Story Mission 8");
        println!("10) Attikus and the Thrall Rebellion - Operation 1");
        println!("11) Toby's Friendship Raid - Operation 2");
        println!("12) Oscar Mike vs the Battleschool - Operation 3");
        println!("13) Montana and the Demon Bear - Operation 4");
        println!("14) Phoebe and the Heart of Ekkunar - Operation 5");

        stdout.flush();
        let mut map_selection: String = "".to_string();
        stdin.read_line(&mut map_selection);

        println!("Type the number of the character you want to play:");
        
        println!("1) Alani");
        println!("2) Ambra");
        println!("3) Attikus");
        println!("4) Beatrix");
        println!("5) Benedict");
        println!("6) Boldur");
        println!("7) Caldarius");
        println!("8) Deande");
        println!("9) El Dragon");
        println!("10) Ernest");
        println!("11) Galilea");
        println!("12) Ghalt");
        println!("13) ISIC");
        println!("14) Kelvin");
        println!("15) Kid Ultra");
        println!("16) Kleese");
        println!("17) Marquis");
        println!("18) Mellka");
        println!("19) Miko");
        println!("20) Montana");
        println!("21) Orendi");
        println!("22) Oscar Mike");
        println!("23) Pendles");
        println!("24) Phoebe");
        println!("25) Rath");
        println!("26) Reyna");
        println!("27) Shayne & Aurox");
        println!("28) Thorn");
        println!("29) Toby");
        println!("30) Whiskey Foxtrot");

        stdout.flush();
        let mut character_selection: String = "".to_string();
        stdin.read_line(&mut character_selection);
        
        write_config_file(map_selection.trim().to_string(), character_selection.trim().to_string());

        println!("Launching BattleBorn...");

        launch_battleborn();

        println!("Waiting for the game to launch...");

        std::thread::sleep(Duration::from_secs(7));

        println!("Injecting ReBorn...");

        inject_reborn();
    }
    else{
        println!("Failed to find your installation of Battleborn: try placing this exe into the root of your Battleborn directory.");
        stdout.flush();
        let mut useless_string = "".to_string();
        stdin.read_line(&mut useless_string);
        return;
    }
    
}

fn inject_reborn(){
    let battleborn = OwnedProcess::find_first_by_name("Battleborn.exe").unwrap();
    let syringe = Syringe::for_process(battleborn);

    let payload = syringe.inject("ReBorn.dll").unwrap();
}

fn launch_battleborn(){
    if(Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe").exists()){
        Command::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe").spawn();
    }
    else if(Path::new("Binaries\\Win64\\Battleborn.exe").exists()){
        Command::new("Binaries\\Win64\\Battleborn.exe").spawn();
    }
}

fn write_config_file(map: String, character: String){
    if(Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe").exists()){
        if(Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\config.txt").exists()){
            remove_file("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\config.txt");
        }

        let mut config_file: File = File::create("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\config.txt").unwrap();
        config_file.write([map, character].join("|").as_bytes());
    }
    else if(Path::new("Binaries\\Win64\\Battleborn.exe").exists()){
        if(Path::new("Binaries\\Win64\\config.txt").exists()){
            remove_file("Binaries\\Win64\\config.txt");
        }

        let mut config_file: File = File::create("Binaries\\Win64\\config.txt").unwrap();
        config_file.write([map, character].join("|").as_bytes());
    }
}

fn locate_binary_and_patch() -> bool{
    //let battleborn_exe = get_file_as_byte_vec(&"C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe".to_string());
    if(Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe").exists()){
        println!("Located Battleborn Binary!");

        let mut battleborn_exe: Vec<u8> = get_file_as_byte_vec(&"C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe".to_string());
        let current_hash = digest(battleborn_exe.clone());
        println!("Hash of current Battleborn Binary: {}", current_hash);

        if(current_hash.contains("1687f394fa650a56f443c371076ad91e1067c22b146dbbe556240abb882354da")){
            println!("Binary is already patched, continuing...");
            return true;
        }
        else if(current_hash.contains("19d27febeb20fc80c1e5cecd12fcf0d3bbb6e32a6a5da08f968110d14646e531")){
            println!("Binary is not patched!");

            battleborn_exe[0x027BF628] = 0x43;
            battleborn_exe[0x027BF62A] = 0x61;
            battleborn_exe[0x027BF62C] = 0x73;
            battleborn_exe[0x027BF62E] = 0x63;
            battleborn_exe[0x027BF630] = 0x61;
            battleborn_exe[0x027BF632] = 0x64;
            battleborn_exe[0x027BF634] = 0x65;

            println!("Patching binary...");
            let mut battleborn_exe_file = OpenOptions::new().read(true).write(true).open("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe").unwrap();
            battleborn_exe_file.write_all(&battleborn_exe);
            println!("Binary patched!");

            return true;
        }
    }
    else{
        if(Path::new("Binaries\\Win64\\Battleborn.exe").exists()){
            println!("Located Battleborn Binary!");

        let mut battleborn_exe: Vec<u8> = get_file_as_byte_vec(&"Binaries\\Win64\\Battleborn.exe".to_string());
        let current_hash = digest(battleborn_exe.clone());
        println!("Hash of current Battleborn Binary: {}", current_hash);

        if(current_hash.contains("1687f394fa650a56f443c371076ad91e1067c22b146dbbe556240abb882354da")){
            println!("Binary is already patched, continuing...");
            return true;
        }
        else if(current_hash.contains("19d27febeb20fc80c1e5cecd12fcf0d3bbb6e32a6a5da08f968110d14646e531")){
            println!("Binary is not patched!");

            battleborn_exe[0x027BF628] = 0x43;
            battleborn_exe[0x027BF62A] = 0x61;
            battleborn_exe[0x027BF62C] = 0x73;
            battleborn_exe[0x027BF62E] = 0x63;
            battleborn_exe[0x027BF630] = 0x61;
            battleborn_exe[0x027BF632] = 0x64;
            battleborn_exe[0x027BF634] = 0x65;

            println!("Patching binary...");
            let mut battleborn_exe_file = OpenOptions::new().read(true).write(true).open("Binaries\\Win64\\Battleborn.exe").unwrap();
            battleborn_exe_file.write_all(&battleborn_exe);
            println!("Binary patched!");

            return true;
        }
        }
    }

    return false;
}