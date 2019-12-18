use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use serde_json::Value;

use websocket::client::{
    sync::{Reader, Writer},
    ClientBuilder,
};
use websocket::OwnedMessage;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Error {
    ConnectionFailed,
    CallFailed,
    NotResponse,
    Json(serde_json::Error),
    Response(String),
}

pub struct Connection {
    outgoing_tx: Sender<String>,
    incoming_rx: Receiver<String>,
}

pub struct Response {
    payload: String,
}

pub struct ZomeCall {
    instance: String,
    zome: String,
    function: String,
    args: Value, 
}
/**
## Connect to the websocket
The websocket connection will need a thread for the sending and recieving of messages.
The [Connection](Connection) should hold channels to these threads.
*/
pub fn connect(address: &str) -> Result<Connection, Error> {
    let (outgoing_tx, outgoing_rx) = channel();
    let (incoming_tx, incoming_rx) = channel();
    let (ws_reciever, ws_sender) = new_websocket(address)?;
    outgoing(outgoing_rx, ws_sender);
    incoming(incoming_tx, ws_reciever);
    Ok(Connection::new(outgoing_tx, incoming_rx))
}

/**
## Create a new connection
The websocket [ClientBuilder](websocket::client::ClientBuilder) can be used to make a connection
to the Holochain conductor websocket.
For this tutorial we are just making an insecure connection.
*/
pub fn new_websocket(
    address: &str,
) -> Result<(Reader<std::net::TcpStream>, Writer<std::net::TcpStream>), Error> {
    ClientBuilder::new(address)
        .map_err(|_| Error::ConnectionFailed)?
        .add_protocol("rust-websocket")
        .connect_insecure()
        .map_err(|_| Error::ConnectionFailed)?
        .split()
        .map_err(|_| Error::ConnectionFailed)
}
/**
Loop through the outgoing calls and send them to the websocket.
*/
fn outgoing(outgoing_rx: Receiver<String>, mut ws_sender: Writer<TcpStream>) {
    thread::spawn(move || {
        for call in outgoing_rx.iter() {
            ws_sender.send_message(&OwnedMessage::Text(call)).ok();
        }
    });
}

fn incoming(incoming_tx: Sender<String>, mut ws_reciever: Reader<TcpStream>) {
    thread::spawn(move || {
        for msg in ws_reciever.incoming_messages() {
            if let Ok(OwnedMessage::Text(msg)) = msg {
                incoming_tx.send(msg).ok();
            }
        }
    });
}

impl ZomeCall {
    pub fn new<T>(instance: T, zome: T, function: T, args: Value) -> Self
    where
        T: Into<String>,
    {
        ZomeCall {
            instance: instance.into(),
            zome: zome.into(),
            function: function.into(),
            args,
        }
    }
    //{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "hello_holo", "args": {} }}
    fn json(&self) -> String {
        let json = serde_json::json!({"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": self.instance, "zome": self.zome, "function": self.function, "args": self.args}});
        let r = json.to_string();
        dbg!(&r);
        r
    }
}

impl Connection {
    /// Store the channels for later
    fn new(outgoing_tx: Sender<String>, incoming_rx: Receiver<String>) -> Self {
        Connection {
            outgoing_tx,
            incoming_rx,
        }
    }

    /**
    ## Calling a zome
    The zome is call through the connection using the
    [ZomeCall](ZomeCall) and then a result is returned with the [Response](Response).
    */
    pub fn call(&self, zome_call: &ZomeCall) -> Result<Response, Error> {
        self.outgoing_tx.send(zome_call.json()).ok();
        for payload in self.incoming_rx.iter() {
            let r = Response { payload };
            if let Err(Error::NotResponse) = r.inner() {
                continue;
            } else {
                return Ok(r);
            }
        }
        Err(Error::ConnectionFailed)
        /*
        self.incoming_rx
            .recv()
            .map_err(|_| Error::ConnectionFailed)
            .map(|payload| Response { payload })
        */
    }
}

impl Response {
    pub fn inner(&self) -> Result<String, Error> {
        let result: Value = serde_json::from_str(&self.payload).map_err(|e| Error::Json(e))?;
        dbg!(&result);
        match result {
            Value::Object(v) => {
                if let Some(Value::String(s)) = v.get("result") {
                    let result = match serde_json::from_str(&s) {
                        Ok(Value::Object(r)) => r,
                        _ => return Err(Error::Response("Invalid return type".into())),
                    };
                    if let Some(result) = result.get("Ok") {
                        Ok(result.to_string())
                    } else if let Some(error) = result.get("Err") {
                        Err(Error::Response(format!("Zome error {:?}", error)))
                    } else {
                        Err(Error::Response("Invalid return type".into()))
                    }
                } else {
                    Err(Error::NotResponse)
                }
            }
            _ => Err(Error::Response("Wrong format from ".into())),
        }
    }
}

impl From<Error> for String {
    fn from(e: Error) -> Self {
        format!("Holochain Error: {:?}", e)
    }
}
