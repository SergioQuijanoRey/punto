use std::fs;

pub struct Handlers {
    pub dir: tempfile::TempDir,
}

/// Create some basic structure so we can start working
/// `tmp_dir.path()`
/// |_ origin/
///      |__ file1.txt
///      |__ file2.txt
///      |____ dir1/
///          |__ a.txt
///          |__ b.txt
///          |__ c.txt
///
pub fn setup_basic_scenario(prefix_name: &str) -> Handlers {
    let tmp_dir = tempfile::Builder::new()
        .prefix(prefix_name)
        .tempdir()
        .expect("Could not create initial temp dir for testing");

    let origin_dir_path = &tmp_dir.path().join("origin");
    fs::create_dir(origin_dir_path).expect("Could not create dir inside temp dir");

    let file_path = &origin_dir_path.join("file1.txt");
    fs::File::create(file_path).expect("Could not create temp file for testing");

    let file_path = &origin_dir_path.join("file2.txt");
    fs::File::create(file_path).expect("Could not create temp file for testing");

    let inner_dir_path = &origin_dir_path.join("dir1");
    fs::create_dir(inner_dir_path).expect("Could not create inner dir inside temp dir");

    for file_name in ["a.txt", "b.txt", "c.txt"] {
        let file_path = &inner_dir_path.join(file_name);
        fs::File::create(file_path).expect("Could not create temp file for testing");
    }

    Handlers { dir: tmp_dir }
}

/// Same as `setup_basic_scenario` but with some files already in destination dir, which can be
/// useful for some testing cases:
///
/// `tmp_dir.path()`
/// |_ origin/
///      |__ file1.txt
///      |__ file2.txt
///      |____ dir1/
///          |__ a.txt
///          |__ b.txt
///          |__ c.txt
/// |_ destination/
///      |__ file1.txt
///      |__ file3.txt
///      |____ dir1/
///          |__ c.txt
///          |__ d.txt
///
pub fn setup_basic_but_messy_scenario(prefix_name: &str) -> Handlers {
    let tmp_dir = tempfile::Builder::new()
        .prefix(prefix_name)
        .tempdir()
        .expect("Could not create initial temp dir for testing");

    // Origin file structure
    let origin_dir_path = &tmp_dir.path().join("origin");
    fs::create_dir(origin_dir_path).expect("Could not create dir inside temp dir");

    let file_path = &origin_dir_path.join("file1.txt");
    fs::File::create(file_path).expect("Could not create temp file for testing");

    let file_path = &origin_dir_path.join("file2.txt");
    fs::File::create(file_path).expect("Could not create temp file for testing");

    let inner_dir_path = &origin_dir_path.join("dir1");
    fs::create_dir(inner_dir_path).expect("Could not create inner dir inside temp dir");

    for file_name in ["a.txt", "b.txt", "c.txt"] {
        let file_path = &inner_dir_path.join(file_name);
        fs::File::create(file_path).expect("Could not create temp file for testing");
    }

    // Messy destination file structure
    let destination_dir_path = &tmp_dir.path().join("destination");
    fs::create_dir(destination_dir_path).expect("Could not create destination dir inside temp dir");

    for file_name in ["file1.txt", "file3.txt"] {
        let file_path = &destination_dir_path.join(file_name);
        fs::File::create(file_path).expect("Could not create temp file for testing");
    }

    let dest_inner_dir_path = &destination_dir_path.join("dir1");
    fs::create_dir(dest_inner_dir_path).expect("Could not create inner dir inside destination dir");

    for file_name in ["c.txt", "d.txt"] {
        let file_path = &dest_inner_dir_path.join(file_name);
        fs::File::create(file_path).expect("Could not create temp file for testing");
    }

    Handlers { dir: tmp_dir }
}
