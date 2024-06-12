use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env::set_current_dir};
use tokio::fs::{create_dir_all, read, read_dir, write};
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Template {
    pub tree: Vec<String>,
    pub bin: HashMap<String, Vec<u8>>,
}

impl Template {
    pub async fn from_dir(path: String) -> Self {
        let mut bin = HashMap::new();
        let mut tree = Vec::new();
        #[async_recursion]
        async fn recursion_tree_dir(
            parent: String,
            path: String,
            tree: &mut Vec<String>,
            bin: &mut HashMap<String, Vec<u8>>,
        ) {
            let mut entries = read_dir(path).await.unwrap();
            while let Some(entry) = entries.next_entry().await.unwrap() {
                let file_type = entry.file_type().await.unwrap();
                let path = entry.path().to_string_lossy().to_string();
                let name = entry.file_name().to_string_lossy().to_string();
                if file_type.is_dir() {
                    tree.push(path.clone());
                    recursion_tree_dir(format!("{}{}/", parent, name), path, tree, bin).await;
                } else if file_type.is_file() {
                    bin.insert(format!("{}{}", parent, name), read(path).await.unwrap());
                }
            }
        }
        recursion_tree_dir("".into(), path, &mut tree, &mut bin).await;
        Self { tree, bin }
    }

    pub async fn from_template_buff(buff: Vec<u8>) -> Self {
        bincode::deserialize(buff.as_slice()).unwrap()
    }

    pub async fn from_template_path(path: String) -> Self {
        Self::from_template_buff(read(path).await.unwrap()).await
    }

    pub async fn generate(&self, target: String) {
        // mkdir
        set_current_dir(target).unwrap();
        for subpath in self.tree.clone() {
            create_dir_all(subpath).await.unwrap();
        }

        for (name, contents) in self.bin.iter() {
            write(name, contents).await.unwrap();
        }
    }
}

#[tokio::test]
async fn test_dir() {
    let temp = Template::from_dir(r#"src"#.into()).await;

    temp.generate("test".into()).await;
}
