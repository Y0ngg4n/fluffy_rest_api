use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::api::websocket::messages::{WsMessage, Disconnect, Connect, ClientActorMessage};
use crate::api::websocket::json_messages::{ScribbleAdd, ScribbleUpdate};
use crate::db::websocket::scribble::{scribble_add, scribble_update};
use std::sync::Arc;
use scylla::Session;
use tokio::task;


type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>,      //room id  to list of users id
    database_session: Arc<Session>

}

impl Lobby {
    pub(crate) fn default(session: &Arc<Session>) -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            database_session: session.clone()
        }
    }
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| self.send_message(&format!("{} disconnected.", &msg.id), user_id));
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new).insert(msg.self_id);

        self
            .rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.self_id), conn_id));

        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );

        self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        // println!("Message: {}", msg.msg);
        if msg.msg.starts_with("\\w") {
            // Send to specific User
            if let Some(id_to) = msg.msg.split('#').collect::<Vec<&str>>().get(1) {
                self.send_message(&msg.msg, &Uuid::parse_str(id_to).expect("Could not parse Message"));
            }
        } else if msg.msg.starts_with("scribble-add#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("scribble-add#", "");
            let parsed: ScribbleAdd = serde_json::from_str(&json).expect("Cant unwrap scribble-add json");
            task::spawn(scribble_add(self.database_session.clone(), parsed, msg.room_id));
        }else if msg.msg.starts_with("scribble-update#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("scribble-update#", "");
            let parsed: ScribbleUpdate = serde_json::from_str(&json).expect("Cant unwrap scribble-update json");
            task::spawn(scribble_update(self.database_session.clone(), parsed));
        }
        else {
            // Broadcast
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| self.send_message(&msg.msg, client));
        }
    }
}