use std::{
    io::{self, BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    sync::mpsc::{sync_channel, SendError, SyncSender},
    thread::{self, JoinHandle},
};

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Deserializer;
use thiserror::Error;

pub struct PhpExec {
    child_process: Child,
    child_stdout_reader: BufReader<ChildStdout>,
    child_stdin_writer_handle: Option<JoinHandle<()>>,
    child_stdin_writer_channel: Option<SyncSender<String>>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Io Error: {0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("{0}")]
    Send(#[from] SendError<String>),
}

pub type Result<T> = std::result::Result<T, Error>;

const EXEC: &'static str = include_str!("exec.php");

impl PhpExec {
    pub fn new() -> Result<Self> {
        let mut child_process = Command::new("php")
            .args(["-r", &EXEC[5..]])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdout = child_process.stdout.take().unwrap();
        let child_stdout_reader = BufReader::new(stdout);

        let mut stdin = child_process.stdin.take().unwrap();
        let (sender, receiver) = sync_channel::<String>(1);
        let handle = thread::spawn(move || {
            if let Ok(s) = receiver.recv() {
                stdin.write_all(s.as_bytes()).unwrap();
                stdin.write_all(b"\n").unwrap();
                stdin.flush().unwrap();
            }
        });

        Ok(Self {
            child_process,
            child_stdin_writer_handle: Some(handle),
            child_stdin_writer_channel: Some(sender),
            child_stdout_reader,
        })
    }

    pub fn exec<R: DeserializeOwned>(&mut self, code: impl AsRef<str>) -> Result<PhpResult<R>> {
        let code = serde_json::to_string(code.as_ref()).unwrap();
        self.child_stdin_writer_channel
            .as_ref()
            .unwrap()
            .send(code)?;
        let mut response_buffer = String::new();
        self.child_stdout_reader.read_line(&mut response_buffer)?;
        let mut des = Deserializer::from_str(&response_buffer);
        Ok(PhpResult::deserialize(&mut des)?)
    }
}

impl Drop for PhpExec {
    fn drop(&mut self) {
        if let Some(channel) = self.child_stdin_writer_channel.take() {
            drop(channel);
        }
        if let Some(handle) = self.child_stdin_writer_handle.take() {
            handle.join().unwrap();
        }
        self.child_process.kill().unwrap();
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PhpResult<R> {
    Ok {
        result: R,
        stdout: String,
    },
    Error {
        message: String,
        code: u64,
        stdout: String,
    },
}

impl<R> From<PhpResult<R>> for std::result::Result<R, String> {
    fn from(value: PhpResult<R>) -> Self {
        value.to_result()
    }
}

impl<R> PhpResult<R> {
    pub fn to_result(self) -> std::result::Result<R, String> {
        match self {
            PhpResult::Ok { result, .. } => Ok(result),
            PhpResult::Error { message, .. } => Err(message),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{PhpExec, PhpResult};

    #[test]
    fn test() {
        let mut exec = PhpExec::new().unwrap();
        let constants: PhpResult<HashMap<String, u32>> = exec
            .exec("get_defined_constants(true)['tokenizer']".to_string())
            .unwrap();

        dbg!(constants);
    }
}
