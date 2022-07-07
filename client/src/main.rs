mod client;


use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use md5::Digest;
use std::{str};


pub fn main() {
    let stream = std::net::TcpStream::connect("localhost:7878");
    match stream {

        Ok(mut stream) => {
            let array = [0; 4];

            let hello = Message::Hello;
            send(&mut stream, hello);

            receive(&mut stream, array);
            receive(&mut stream, array);
            receive(&mut stream, array);
            receive(&mut stream, array);
            receive(&mut stream, array);



        }
        Err(err) => panic!("Cannot connect: {err}")
    }
}


fn send(stream: &mut TcpStream, message_to_send: Message) {
    let message_to_serialized = serde_json::to_string(&message_to_send);
    let message_to_serialized = message_to_serialized.unwrap();
    let serialized_message_length_to_u32 = (message_to_serialized.len()) as u32;

    stream.write_all(&serialized_message_length_to_u32.to_be_bytes()).unwrap();

    stream.write_all(&message_to_serialized.as_bytes()).unwrap();

}

fn receive(stream: &mut TcpStream, mut array: [u8; 4]) {
    stream.read( &mut array).unwrap();

    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;
    let mut vector = vec![0; size_message];

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
            let subscribe = Message::Subscribe(Subscribe { name: "Warda".to_string()});
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
                    println!("Result:{:?}",input.message);



                } }


        }
        Message::EndOfGame(input) => {

        }
        Message::PublicLeaderBoard(subscribe) => {
            println!("Result:{:?}",subscribe);

        }
        _ => {}
    }


}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome{
    version: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe{
    name: String
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
     ChallengeAnswer(ChallengeAnswer),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
    MD5HashCashInput(MD5HashCashInput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicLeaderBoard(Vec<PublicPlayer>);

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult(BadResult),
    OK(Ok)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadResult {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ok {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    complexity: u32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame {
    leader_board: PublicLeaderBoard,
}






