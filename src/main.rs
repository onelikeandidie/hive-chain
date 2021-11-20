use std::sync::{Arc, Mutex};

use rand::{Rng, distributions::Alphanumeric, prelude::SliceRandom, thread_rng};
use hivechain::{Block, Blockchain, Hashable, Transaction, transaction::{self, Output}, util::now};

#[tokio::main]
async fn main() {
    let mut block = Block::new(
        0, 
        now(), 
        vec![0;32], 
        vec![
            Transaction { 
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        address: "Alice".to_owned(),
                        value: 5000
                    },
                    transaction::Output {
                        address: "Bob".to_owned(),
                        value: 5000
                    },
                    transaction::Output {
                        address: "Chris".to_owned(),
                        value: 5000
                    }
                ]
            }
        ],
        0x000FFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    );
    block.hash();
    block.mine();

    println!("{:?}", block);

    let mut blockchain = Blockchain::default();

    match blockchain.add(block) {
        Ok(_) => {
            println!("Genesis Block Added")
        },
        Err(error) => {
            println!("Genesis Block Error \n{:?}", error)
        },
    }

    let blockchain = Arc::new(Mutex::new(blockchain));
    let thread_blockchain = blockchain.clone();

    let bench_handle = tokio::spawn(bench_blocks(10, thread_blockchain, "Miner".to_owned()));
    
    bench_handle.await.unwrap();
}

async fn bench_blocks (amount: u32, blockchain: Arc<Mutex<Blockchain>>, miner_address: String) {
    // Loop lol
    let mut i: u32 = 0;
    loop {
        // Lock the blockchain
        let mut blockchain = blockchain.lock().unwrap();
        match blockchain.get_last_block() {
            Some(last_block) => {
                // Create an address to send to
                let address: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(6)
                    .map(char::from)
                    .collect();
                // Define how much fee the miner will get from the block
                let fee = blockchain.get_fee() as u64;
                // Collect all outputs from last block
                let miner_address = miner_address.clone();
                let outputs = last_block.transactions
                    .iter()
                    .flat_map(|transaction| {
                        transaction.outputs
                            .iter()
                            // Filter ones that have money
                            .filter(|output| output.value > fee)
                            // Filter the ones that arent the miner
                            .filter(|output| output.address != miner_address)
                            .map(|output| output.clone())
                            .collect::<Vec<Output>>()
                    })
                    .collect::<Vec<Output>>();
                // Get a random input (output from last block)
                println!("Possible Outputs {:?}", outputs);
                let random_input = outputs.choose(&mut thread_rng()).unwrap();
                // Get the current difficulty
                let difficulty = blockchain.get_difficulty();
                // Create a random value moved
                let random_coins_spent_value = rand::thread_rng()
                    .gen_range(0..(random_input.value));
                // Create a random output
                let random_output = Output {
                    value: random_coins_spent_value,
                    address
                };
                // Update the input wallet with an output
                let mut random_input_updated = random_input.clone();
                random_input_updated.value -= random_coins_spent_value + fee;
                // Create the payload
                let payload = vec![
                    Transaction {
                        inputs: vec![],
                        outputs: vec![
                            Output {
                                address: miner_address.clone(),
                                value: fee
                            }
                        ]
                    },
                    Transaction {
                        inputs: vec![
                            random_input.clone()
                        ],
                        outputs: vec![
                            random_input_updated.clone(), 
                            random_output.clone()
                        ]
                    }
                ];
                // Create a block with the payload
                let mut block = Block::new(
                    last_block.index + 1, 
                    now(), 
                    last_block.hash.clone(), 
                    payload.clone(),
                    difficulty
                );
                println!("{:?}", payload);
                println!("Mining: {:?}", block.index);
                // Keep track of when it started mining
                let before = now();
                // Mine it
                block.mine();
                // Check how much time passed
                let after = now();
                let time_it_took_to_mine = after - before;
                println!("Mined {}(...) at {}, took {} ms", 
                    &hex::encode(&block.hash).to_string()[..4],
                    block.index,
                    time_it_took_to_mine
                );
                // Add it to the blockchain
                match blockchain.add(block) {
                    Ok(_) => {
                        i += 1;
                    },
                    Err(error) => {
                        println!("Error with adding block to blockcahin\n{:?}", error);
                        println!("Trying again");
                    },
                }
            },
            None => todo!(),
        }
        if i >= amount {
            break;
        }
    }
}
