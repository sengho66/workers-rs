use crate::{error::Error, helpers::random_color, message::MsgResponse};

use std::{collections::HashMap, rc::Rc, sync::RwLock};
use wasm_bindgen::prelude::wasm_bindgen;
use worker::{wasm_bindgen, WebSocket};

pub type UsersMap = HashMap<Box<str>, User>;

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct Users(pub(crate) Rc<RwLock<UsersMap>>);

#[wasm_bindgen]
impl Users {
    /// Broadcasts a message to all connected clients.
    pub(crate) fn broadcast(&self, msg: &MsgResponse) {
        // iterates connected clients to send the message
        if let Ok(users) = self.0.read() {
            for (id, session) in &*users {
                if session.session.send_with_str(msg.as_str()).is_err() {
                    self.remove(id);
                }
            }
        }
    }

    /// Add a new user to the session.
    pub(crate) fn add(&self, user: User) -> Result<(), Error> {
        self.0.write()?.insert((&*user.info.id).into(), user);
        Ok(())
    }

    /// Close the connection from the server.
    pub fn close(&self, id: &str) {
        if let Some(user) = self.0.read().unwrap().get(id) {
            let _ = user.session.close(
                Some(1000),
                Some(&format!(
                    "Reason: Client hasn't responded since {} heartbeats.",
                    user.hb
                )),
            );
        }
    }

    /// Removes a user corresponding to the id.
    pub(crate) fn remove(&self, id: &str) {
        if let Ok(users) = self.0.write().as_mut() {
            users.remove(id);
        }
    }

    /// Returns its current heartbeat's count.
    /// None if client lost connection.
    pub fn hb(&self, id: &str) -> Option<usize> {
        if let Some(user) = self.0.read().unwrap().get(id) {
            return Some(user.hb);
        }
        None
    }

    /// Sends "ping" to client.
    pub fn ping(&self, id: &str) -> Option<usize> {
        if let Some(user) = self.0.write().unwrap().get_mut(id) {
            if user.session.send_with_str("ping").is_ok() {
                user.hb += 1;
            }
            return Some(user.hb);
        }
        None
    }

    pub(crate) fn pong(&self, id: &str) {
        if let Some(user) = self.0.write().unwrap().get_mut(id) {
            user.hb = 0;
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub hb: usize,
    pub info: UserInfo,
    pub session: WebSocket,
}

impl User {
    pub fn new(id: String, name: String, session: WebSocket) -> Self {
        Self {
            hb: 0,
            info: UserInfo::new(id, name),
            session,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserInfo {
    /// Unique color of the user's name
    pub color: String,
    /// User's unique id
    pub id: String,
    /// User's name
    pub name: String,
}

impl UserInfo {
    fn new(id: String, name: String) -> Self {
        Self {
            color: random_color(),
            id,
            name,
        }
    }
}
