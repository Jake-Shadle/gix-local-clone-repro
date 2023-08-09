const REMOTE: &str = "https://github.com/EmbarkStudios/cpal";
const REV: &str = "47b0ff833c562d0a95e5d5ea3706235156bb54ce";

use gix::progress::Discard;
use gix::remote::Direction::Fetch as DIR;

fn main() {
    let temp_dir = tempfile::tempdir().unwrap();

    // Clone via HTTP
    let (repo, _out) = {
        gix::prepare_clone_bare(REMOTE, temp_dir.path())
            .expect("failed to prepare clone")
            .with_remote_name("origin")
            .unwrap()
            .configure_remote(|remote| {
                Ok(remote
                    .with_fetch_tags(gix::remote::fetch::Tags::All)
                    .with_refspecs(["+HEAD:refs/remotes/origin/HEAD"], DIR)?)
            })
            .fetch_only(&mut Discard, &Default::default())
            .expect("failed to fetch")
    };

    let rev_id = gix::ObjectId::from_hex(REV.as_bytes()).unwrap();

    // Ensure that the repo actually contains the revision we need
    repo.find_object(rev_id)
        .expect("unable to find commit in fresh clone");

    // Now do a clone to a different local directory
    let local_clone = tempfile::tempdir().unwrap();

    // Note: won't work on windows...
    let (other_repo, _out) = gix::prepare_clone(
        format!("file://{}", temp_dir.path().display()),
        local_clone.path(),
    )
    .expect("failed to prepare clone")
    .with_remote_name("origin")
    .unwrap()
    .configure_remote(|remote| {
        Ok(remote
            .with_fetch_tags(gix::remote::fetch::Tags::All)
            .with_refspecs(["+refs/heads/*:refs/remotes/origin/*"], DIR)?)
    })
    .fetch_only(&mut Discard, &Default::default())
    .expect("failed to clone from local db");

    other_repo
        .find_object(rev_id)
        .expect("couldn't find id in local clone :(");
}
