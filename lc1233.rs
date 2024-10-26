// LeetCode problem 1233: Remove Sub-Folders From the Filesystem
// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/description/

fn main() {
    let result = remove_subfolders(vec![
        "/a".into(),
        "/a/b".into(),
        "/c/d".into(),
        "/c/d/e".into(),
        "/c/f".into(),
    ]);
    let result = remove_subfolders(vec!["/a".into(), "/a/b/c".into(), "/a/b/d".into()]);
    println!("Result = {result:?}");
}

fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    let mut directories = folder
        .iter()
        .map(|v| v[1..].split("/").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    directories.sort_unstable_by_key(|x| x.len());

    println!("{:?}", directories);
    let mut root_folders = std::collections::HashSet::new();

    for d in directories {
        let mut existing = false;
        let mut current = String::new();

        for s in d {
            current += &format!("/{s}");

            if root_folders.contains(&current) {
                existing = true;
                break;
            }
        }

        if !existing {
            root_folders.insert(current);
        }
    }

    return root_folders.into_iter().collect::<Vec<_>>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = remove_subfolders(vec![
            "/a".into(),
            "/a/b".into(),
            "/c/d".into(),
            "/c/d/e".into(),
            "/c/f".into(),
        ]);
        let expected: Vec<String> = vec!["/a".into(), "/c/d".into(), "/c/f".into()];
        assert_eq!(result, expected);
    }

    #[test]
    fn example_two() {
        let result = remove_subfolders(vec!["/a".into(), "/a/b/c".into(), "/a/b/d".into()]);
        let expected: Vec<String> = vec!["/a".into()];
        assert_eq!(result, expected);
    }

    #[test]
    fn example_three() {
        let result = remove_subfolders(vec!["/a/b/c".into(), "/a/b/ca".into(), "/a/b/d".into()]);
        let expected: Vec<String> = vec!["/a/b/c".into(), "/a/b/ca".into(), "/a/b/d".into()];
        assert_eq!(result, expected);
    }
}
