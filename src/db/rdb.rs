pub mod rdb{
    use std::collections::HashMap;

use uuid::Uuid;

    #[derive(Debug,Clone)]
    pub struct Credentials{
        pub uid: Uuid,
        pub username: String,
        password: Option<String>,
        pub displayed_name: String,
        pub peers: Peers,        // <----- These two fields are subject to TOCTOU and desync
        pub peer_map: Peer_Maps  // <-|,
    }

    impl Credentials{
        pub fn new(username: Option<String>,password: Option<String>,name: Option<String>) -> Self{
            let username =  username.unwrap_or("ANONYMOUS".to_string());
            Self { uid: Uuid::new_v4(), username:username.clone(), password: if &username[..] == "ANONYMOUS"{None}else{if password.is_none(){panic!("Usernames require Passwords for SignIn")}else{password}}, displayed_name: name.unwrap_or(username.clone()), peers: Peers::default(), peer_map: Peer_Maps::default() }
        }
    }

    impl Drop for Credentials{
        fn drop(&mut self) {
            self.password = None;
            todo!()
        }
    }





    #[derive(Debug,Clone)]
    pub struct Peers{
        id2peer: HashMap<Uuid,String>,
        peer2id: HashMap<String,Uuid>
    }

    impl Default for Peers{
        fn default() -> Self {
            Self { id2peer: HashMap::new(), peer2id: HashMap::new() }
        }
    }


    impl Peers{
        
        pub fn new() -> Self{
            Peers::default()
        }


        pub fn is_peer(&self,peer: &str) -> Option<&Uuid>{
            self.peer2id.get(peer)
        }

        pub fn is_id_peer(&self,uuid:&Uuid) -> Option<&String>{
            self.id2peer.get(uuid)
        }

        pub fn add_peer(&mut self,id:&Uuid,peer:&str) -> bool{
            self.peer2id.insert(peer.to_owned(), id.clone());
            self.id2peer.insert( id.clone(),peer.to_owned());
            true
        }
    }

    #[derive(Debug,Clone)]
    pub struct Peer_Maps{
        pub Ip_Map: HashMap<Uuid,Vec<String>>
    }

    impl Default for Peer_Maps{
        fn default() -> Self {
            Self { Ip_Map: HashMap::new() }
        }
    }

    impl Peer_Maps{
        pub fn new() -> Self{
            Peer_Maps::default()
        }
    

        pub fn add_conn(&mut self,uuid: Uuid,conn: String){
            self.Ip_Map.entry(uuid).or_insert(vec![]).push(conn);
        }

    }




}