extern crate imap;
extern crate native_tls;
extern crate termios;

use std::os::unix::io::AsRawFd;
use std::io::Write;

fn input_pass() -> String {
    let stdin = std::io::stdin().as_raw_fd();
    let term = termios::Termios::from_fd(stdin).unwrap();
    let mut term_hidden_input = term;
    term_hidden_input.c_lflag &= !termios::ECHO;
    let mut pass = String::new();
    write!(std::io::stdout(), "password:").ok();
    std::io::stdout().flush().ok();
    termios::tcsetattr(stdin, termios::TCSANOW, &term_hidden_input).unwrap();
    std::io::stdin().read_line(&mut pass).ok();
    termios::tcsetattr(stdin, termios::TCSANOW, &term_hidden_input).unwrap();
    pass.trim_end_matches("\n").to_owned()
}

fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = "outlook.office365.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();
    let pass = input_pass();
    let mut imap_session = client.login("s2013553@s.tsukuba.ac.jp", &pass).map_err(|e| e.0)?;
    imap_session.select("Inbox")?;
    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    }
    else {
        return Ok(None);
    };
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_owned();
    imap_session.logout()?;
    Ok(Some(body))
}

fn main() {
    println!("{}", fetch_inbox_top().unwrap_or(None).unwrap_or(String::from("No message")));
}
