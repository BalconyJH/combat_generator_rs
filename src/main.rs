use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncWriteExt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Action {
    pre_delay: u32,
    post_delay: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Combat {
    mode: String,
    role: String,
    version: String,
    combat: Vec<Action>,
}

async fn write_json_file_async(combat: &Combat, path: &str) -> Result<(), io::Error> {
    let mut file = File::create(path).await?;
    let contents = serde_json::to_string_pretty(combat)?;
    file.write_all(contents.as_bytes()).await?;
    Ok(())
}

async fn read_json_file_async(path: &str) -> Result<Combat, io::Error> {
    let contents = tokio::fs::read_to_string(path).await?;
    let combat: Combat = serde_json::from_str(&contents)?;
    Ok(combat)
}

#[tokio::main]
async fn main() {
    let combat = Combat {
        mode: "ExampleMode".to_string(),
        role: "ExampleRole".to_string(),
        version: "1.0".to_string(),
        combat: vec![
            Action { pre_delay: 100, post_delay: 200 },
        ],
    };

    let write_path = "combat.json";
    if let Err(e) = write_json_file_async(&combat, write_path).await {
        eprintln!("Error writing file: {}", e);
    }

    let read_path = "combat.json";
    match read_json_file_async(read_path).await {
        Ok(combat_read) => {
            println!("Read combat data: {:?}", combat_read);
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_write_and_read_json_file() {
        let combat_test = Combat {
            mode: "Test".to_string(),
            role: "Tester".to_string(),
            version: "1.0".to_string(),
            combat: vec![
                Action { pre_delay: 100, post_delay: 200 },
            ],
        };

        let test_path = "test_combat.json";

        // 测试写入功能
        let write_result = write_json_file_async(&combat_test, test_path).await;
        assert!(write_result.is_ok(), "Failed to write to file: {:?}", write_result.err());

        // 测试读取功能
        let read_result = read_json_file_async(test_path).await;
        assert!(read_result.is_ok(), "Failed to read from file: {:?}", read_result.err());

        // 验证内容是否一致
        let combat_read = read_result.expect("Failed to unwrap read result");
        assert_eq!(combat_read, combat_test, "Read data does not match written data");

        // 清理测试文件
        let remove_result = tokio::fs::remove_file(test_path).await;
        assert!(remove_result.is_ok(), "Failed to remove test file");
    }
}
