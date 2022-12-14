
use git2::Repository;

fn main() {

    // Get the github username, repo name and an optional directory name to clone into. 
    let username = std::env::args().nth(1).expect("Provide a valid username");
    let repo_name = std::env::args().nth(2).expect("Provide a valid repo name");
    let dir_name = match std::env::args().nth(3) {
        Some(name) => name,
        None => format!("{}_clone", repo_name)
    };

    // format the url based on username, repo name
    let url = format!("https://github.com/{}/{}", username, repo_name).to_string();

    // Try cloning:
    //      if Ok()  : clone the repository,
    //      if Err() : Display, error mrssage
    if let Ok(_repo) = Repository::clone(url.as_str(), dir_name) {
        println!("Repository {}/{} cloned successfully", username, repo_name);
    } else {
        println!("Error: Check for conflicting directories, validate repo name and user name");
    }
}
