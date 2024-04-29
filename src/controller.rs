use std::process::{Command, Stdio};
use std::io::BufReader;
use std::sync::mpsc;

use crate::Session;
use crate::TcpStream;
use std::io::Read;
use tokio::io;



#[tokio::main]
pub async fn con(tx: &str, user: &str)-> io::Result<String> {
    // Connect to the remote SSH server
    let tcp = TcpStream::connect(tx).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // Try to authenticate with the first identity in the agent.
    // ssh certificate already on remote machine
    sess.userauth_agent(user).unwrap();

    // Make sure we succeeded
    assert!(sess.authenticated());

    // linux/unix 'ls'
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    // linux/unix 'uname -a'
    let mut channel = sess.channel_session().unwrap();
    channel.exec("uname -a").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    let _ = channel.wait_close();

    match channel.exit_status() {
        Ok(0) => println!("{:?}", "Quit ok!"),
        _ => println!("{:?}", "error closing conn"),
    }
    Ok(s)
}
