mod client;


use std::io::{Read, Write};
use std::net::TcpStream;
use shared::{Challenge, ChallengeAnswer, ChallengeResult, MD5HashCashOutput, Message, Subscribe};


pub fn main() {
    let stream = std::net::TcpStream::connect("localhost:7878");
    match stream {

        Ok(mut stream) => {
            let array = [0; 4];

            let hello = Message::Hello;
            send(&mut stream, hello);

            receive(&mut stream, array);
            receive(&mut stream, array);




        }
        Err(err) => panic!("Cannot connect: {err}")
    }
}


fn send(stream: &mut TcpStream, message_to_send: Message) {
    let message = serde_json::to_string(&message_to_send);
    let message_serialized = message.unwrap();
    let message_length = (message_serialized.len()) as u32;

    if let Err(err) = stream.write_all(&message_length.to_be_bytes())
    {
        println!("{err}");
    }

    if let Err(err) =  stream.write_all(&message_serialized.as_bytes())
    {
        println!("{err}");
    }
}

fn receive(stream: &mut TcpStream, mut array: [u8; 4]) {
    if let Err(err) = stream.read( &mut array)
    {
        println!("{err}");
    }

    let message_length_from_bytes: u32 = u32::from_be_bytes(array);
    let message_length = message_length_from_bytes as usize;
    let mut vector = vec![0; message_length];

    if let Err(err) = stream.read(&mut vector)
    {
        println!("{err}");
    }

    let message_received = std::str::from_utf8(&*vector).unwrap();
    println!("received: {}", message_received);

    let message: Message = serde_json::from_str(&message_received).unwrap();


    match message {
        Message::Hello => {}
        Message::Welcome(_) => {
            let subscribe = Message::Subscribe(Subscribe { name: "Warda_bis".to_string()});
            send(stream, subscribe);
        }
        Message::Subscribe(_) => {

        }
        Message::SubscribeResult(subscribe) => {
            println!("Result:{:?}",subscribe);
        }
        Message::Challenge(challenge) => {
            println!("Result:{:?}",challenge);
            match challenge {
                Challenge::MD5HashCash(input) => {
                    let md5 = md5hash_cash(input.comp, input.message);
                    let answer = ChallengeAnswer::MD5HashCash(MD5HashCashOutput { seed: md5.0 as u64, hashcode: md5.1 });
                    let result = ChallengeResult { answer, next_target: "".to_string() };
                    let message = Message::ChallengeResult(result);
                    send(stream, message);
                }}
        }
        Message::EndOfGame(_input) => {

        }
        Message::PublicLeaderBoard(subscribe) => {
            println!("Result:{:?}",subscribe);

        }
        _ => {}
    }

}


fn check_seed(hash: String, comp: u32) -> bool {
    let mut index = 0;
    for c in hash.chars() {
        if c == '1' && index < comp {
            print!("false\n");
            return false;
        } else if index >= comp {
            print!("succeed");
            return true;
        }
        index += 1;
    }
    return false;
}

fn to_binary(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };
    return String::from(b);
}



pub fn md5hash_cash(comp: u32, message: String) -> (u64, String) {
    let mut result = false;
    let mut seed = 0;
    let mut hash_code: String = "".to_string();

    while result == false {
        let trans_seed =format!("{}{}\n", seed, message);//concatenate the seed
        let digest = md5::compute(trans_seed);
        hash_code = format!("{:032X}", digest);
        let hash: String = hash_code.chars().map(to_binary).collect();
        result = check_seed(hash, comp);
        seed += 1;
    }
    return (seed as u64, hash_code);
}
