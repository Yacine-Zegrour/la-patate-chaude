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
    if let Err(err) = stream.read(&mut array)
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
            let subscribe = Message::Subscribe(Subscribe { name: "Wardaaa".to_string() });
            send(stream, subscribe);
        }
        Message::Subscribe(_) => {}
        Message::SubscribeResult(subscribe) => {
            println!("Result:{:?}", subscribe);
        }
        _ => {}
    }
}








