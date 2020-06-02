use std::env;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn random_string(length: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect();

    return rand_string
}

// fn dir_list(path: String, allfile) {
//     filelist = os.listdir(path)
//     for filename in filelist:
//         filepath = os.path.join(path, filename)
//         if os.path.isdir(filepath):
//             dirlist(filepath, allfile)
//         else:
//             allfile.append(filepath)
//     return allfile
// }

// fn handle_git_stash() -> String {
//     let filename: String = random_string(20)
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    let rand_string = random_string(0x20);
    println!("rand_string {}", rand_string)
}
