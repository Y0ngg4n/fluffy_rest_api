use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::api::websocket::messages::{WsMessage, Disconnect, Connect, ClientActorMessage};
use crate::api::websocket::json_messages::{ScribbleAdd, ScribbleUpdate, ScribbleDelete, UploadAdd, UploadUpdate, UploadDelete, TextItemAdd, TextItemUpdate, TextItemDelete, UploadImageDataUpdate, UserMove, UserMoveSend};
use crate::db::websocket::scribble::{scribble_add, scribble_update, scribble_delete};
use std::sync::Arc;
use scylla::Session;
use tokio::task;
use crate::db::websocket::upload::{upload_add, upload_update, upload_delete, upload_image_data_update};
use crate::db::websocket::textitem::{text_item_add, text_item_update, text_item_delete};


type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>,
    //room id  to list of users id
    user: HashMap<Uuid, Uuid>,
    usernames: HashMap<Uuid, String>,
    database_session: Arc<Session>,
}

impl Lobby {
    pub(crate) fn default(session: &Arc<Session>) -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            user: HashMap::new(),
            usernames: HashMap::new(),
            database_session: session.clone(),
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
                    self.user.remove(&msg.id);
                    self.usernames.remove(&msg.id);
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

        self.user
            .entry(msg.self_id.clone())
            .or_insert(msg.user.clone());

        self.usernames
            .entry(msg.self_id.clone())
            .or_insert(msg.username.clone());

        self
            .rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| self.send_message(&format!("user-join#{}#{}#", msg.user, msg.username), conn_id));

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
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        } else if msg.msg.starts_with("scribble-update#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("scribble-update#", "");
            let parsed: ScribbleUpdate = serde_json::from_str(&json).expect("Cant unwrap scribble-update json");
            task::spawn(scribble_update(self.database_session.clone(), parsed));
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        } else if msg.msg.starts_with("scribble-delete#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("scribble-delete#", "");
            let parsed: ScribbleDelete = serde_json::from_str(&json).expect("Cant unwrap scribble-delete json");
            task::spawn(scribble_delete(self.database_session.clone(), parsed));
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        }else if msg.msg.starts_with("upload-add#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("upload-add#", "");
            let parsed: UploadAdd = serde_json::from_str(&json).expect("Cant unwrap upload-add json");
            task::spawn(upload_add(self.database_session.clone(), parsed, msg.room_id));
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        }else if msg.msg.starts_with("upload-update#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("upload-update#", "");
            let parsed: UploadUpdate = serde_json::from_str(&json).expect("Cant unwrap upload-update json");
            task::spawn(upload_update(self.database_session.clone(), parsed));
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        }else if msg.msg.starts_with("upload-image-data-update#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("upload-image-data-update#", "");
            let parsed: UploadImageDataUpdate = serde_json::from_str(&json).expect("Cant unwrap upload-image-data-update json");
            task::spawn(upload_image_data_update(self.database_session.clone(), parsed));
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        }else if msg.msg.starts_with("upload-delete#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("upload-delete#", "");
            let parsed: UploadDelete = serde_json::from_str(&json).expect("Cant unwrap upload-delete json");
            task::spawn(upload_delete(self.database_session.clone(), parsed));
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        }else if msg.msg.starts_with("textitem-add#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("textitem-add#", "");
            let parsed: TextItemAdd = serde_json::from_str(&json).expect("Cant unwrap textitem-add json");
            task::spawn(text_item_add(self.database_session.clone(), parsed, msg.room_id));
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        } else if msg.msg.starts_with("textitem-update#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("textitem-update#", "");
            let parsed: TextItemUpdate = serde_json::from_str(&json).expect("Cant unwrap textitem-update json");
            task::spawn(text_item_update(self.database_session.clone(), parsed));
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        }else if msg.msg.starts_with("textitem-delete#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("textitem-delete#", "");
            let parsed: TextItemDelete = serde_json::from_str(&json).expect("Cant unwrap textitem-delete json");
            task::spawn(text_item_delete(self.database_session.clone(), parsed));
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                self.send_message(&msg.msg, client)
            });
        }else if msg.msg.starts_with("connected-users#") {
            // self.rooms.get(&msg.room_id).unwrap().
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                let user = self.user.get(client).unwrap();
                let username = self.usernames.get(client).unwrap();
                if client.clone() != msg.user.clone() {
                    println!("Send User join");
                    self.send_message(&format!("user-join#{}#{}#", user, username), &msg.id)
                }
            });
        }else if msg.msg.starts_with("user-move#") {
            // self.rooms.get(&msg.room_id).unwrap().
            let json = msg.msg.replace("user-move#", "");
            let parsed: UserMove = serde_json::from_str(&json).expect("Cant unwrap user-move json");
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| if client.clone() != msg.id {
                let user_move_send = UserMoveSend{
                    uuid: msg.user,
                    offset_dx: parsed.offset_dx,
                    offset_dy: parsed.offset_dy,
                    scale: parsed.scale
                };
                let mut message = String::from("user-move#");
                message.push_str(serde_json::to_string(&user_move_send).unwrap().as_str());
                self.send_message(message.as_str(), client)
            });
        }else {
            // Broadcast
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| self.send_message(&msg.msg, client));
        }
    }
}