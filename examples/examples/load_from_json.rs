use serde::{Deserialize, Serialize};
use std::io::Write;

///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Dependency {
    #[serde(rename = "Index")]
    index: usize,

    #[serde(rename = "Dependencies")]
    dependencies: Vec<usize>,
}

fn main() {
    let mut tree = ksq::Tree::from(&[1, 1, 1, 1, 1, 1, 1, 1, 0]).unwrap();
    const TREE_PER_ROW: usize = 100_000;
    const TREE_TOTAL: usize = TREE_PER_ROW * TREE_PER_ROW;

    assert!(tree.bits() >= TREE_TOTAL);

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let dep: Dependency = serde_json::from_str(&line).unwrap();

        eprintln!("{}", dep.index);
        assert!(dep.index <= TREE_PER_ROW);
        let dep_base = dep.index * TREE_PER_ROW;

        for dep_idx in dep.dependencies {
            assert!(dep_idx <= TREE_PER_ROW);
            tree.set(dep_base + dep_idx);
        }
    }

    let mut out = std::fs::File::create("graph.k2").unwrap();
    for chunk in tree.to_vec() {
        out.write(&chunk.to_be_bytes()).unwrap();
    }
}
