use git2::Repository;
use git2::Sort;

fn main() {

    let repo = match Repository::open("/home/foobar/crate-thing/idx") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };


    let mut walk = match repo.revwalk() {
        Ok(walk) => walk,
        Err(e) => panic!("failed to walk: {}", e),
    };    

    walk.push_head().unwrap();

    walk.set_sorting(git2::Sort::REVERSE & git2::Sort::TIME);

    for commit in walk {
        let commit = repo.find_commit(commit.unwrap()).unwrap();

        let msg = commit.message().unwrap();
        let tt = commit.time();

        println!("{}: {}", tt.seconds(), msg);
        
    }
    
}
