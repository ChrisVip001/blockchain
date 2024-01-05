use super::blockchain::Blockchain;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// Block 代表区块链中的区块结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // 保存的当前区块的索引
    pub index: u64,
    // 当前区块的创建时间
    pub timeStamp: u64,
    // 区块的工作证明
    pub proof_of_work: u64,
    // 前一个区块哈希
    pub previous_hash: String,
    // 当前区块哈希
    pub hash: String,
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();

        // 计算生成 SHA-256 哈希值
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // 创建一个新区块，会自动计算并设置哈希
    pub fn new(index: u64, previous_hash: String) -> Self {
        // 当前要创建的区块
        let mut block = Block {
            index,
            timeStamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash,
            hash: String::default(),
        };

        block
    }

    // 挖崛区块哈希
    pub fn mine(&mut self, blockchain: Blockchain) {
        loop {
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                self.proof_of_work += 1;
                self.hash = self.generate_block_hash();
            } else {
                break;
            }
        }
    }

    // 计算块哈希.
    pub fn generate_block_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();
        // Convert block to JSON format.
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();

        // Calculate and return SHA-256 hash value.
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);

        let result = hasher.finalize();

        format!("{:x}", result)
    }
}
