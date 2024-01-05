use chrono::prelude::*;
// internal module
use super::block::Block;

type Blocks = Vec<Block>;

// Blockchain 代表区块链的结构体
#[derive(Debug, Clone)]
pub struct Blockchain {
    // 添加到链中的第一个区块
    pub genesis_block: Block,
    // 存储区块
    pub chain: Blocks,
    // 验证一个区块需要的最小工作量
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        // 链中的第一个块
        let mut genesis_block = Block {
            index: 0,
            time_stamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default()
        };
        // 从创世链开始创建链
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());
  
        // 创建一个区块链实例
        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty
        };
        blockchain
    }

    //添加新的区块链
    pub fn add_block(&mut self) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() - 1].hash.clone()
        );

        new_block.mine(self.clone());
        self.chain.push(new_block.clone());
        println!("New block added to chain -> {:?}", new_block);
    }
}