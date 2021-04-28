use std::collections::HashMap;

type DeviceId = String;
type AccessToken = String;

#[derive(Default)]
pub struct AccessPeer {
    access_token: AccessToken,
    device_id: DeviceId,
}

pub struct AccessManager {
    peers: HashMap<DeviceId, AccessToken>,
}

impl AccessManager {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    // TODO
    pub fn add_user(&mut self, device_id: Option<DeviceId>) -> AccessPeer {
        let mut peer = AccessPeer::default();

        match device_id {
            Some(id) => {
                peer.device_id = id;
            }
            None => {
                peer.device_id = self.generate_device_id();
            }
        }

        peer.access_token = self.generate_access_token();
        peer
    }

    fn add(&mut self, peer: AccessPeer) {
        self.peers.insert(peer.device_id, peer.access_token);
    }

    pub fn generate_access_token(&self) -> AccessToken {
        String::from("") // TODO
    }

    pub fn generate_device_id(&self) -> DeviceId {
        String::from("") // TODO
    }
}
