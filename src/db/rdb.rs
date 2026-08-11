pub mod rdb{
    use std::collections::{HashMap, HashSet};

use uuid::Uuid;
use chrono::{DateTime, NaiveDateTime, TimeDelta, Utc};
use mysql::{params,prelude::*,Pool,PooledConn,TxOpts};

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

    pub type Logs = Vec<Log>;
    pub type Hashes = String;
    pub struct Log{
        pub comm_type: (bool,bool,bool,bool),
        pub duration: TimeDelta,
        pub involved: Vec<Uuid>,
        pub hash: Hashes,
        pub is_success: bool,
        pub chat_history: Vec<(Uuid,DateTime<Utc>,String)>
    }

    impl Default for Log{
        fn default() -> Self {
            Self { comm_type: (false,false,false,true), duration: TimeDelta::zero(), involved: vec![], hash: "".to_string(), is_success:false, chat_history: vec![] }
        }

    }

    impl Log{
        pub fn new() -> Self{
            Self::default()
        }
    }









pub struct Database {
    pool: Pool,
}

impl Database {

 //-------------------------------------------------Init APIs----------------------------------------------------
    pub fn new(url: &str) -> mysql::Result<Self> {
        Ok(Self {pool: Pool::new(url)?})
    }

    fn conn(&self) -> mysql::Result<PooledConn> {
        self.pool.get_conn()
    }

    pub fn init_dbs(&self) -> mysql::Result<()> {
        let mut conn = self.conn()?;
        let mut tx = conn.start_transaction(TxOpts::default())?;
            for i in std::fs::read_to_string("~/RVoic3/src/db/init.sql").unwrap().split(";"){
                tx.exec_drop(i.trim(), ())?;
            }
        tx.commit()?;
        Ok(())
    }

    pub fn clear(&self) -> mysql::Result<()>{
        let mut conn = self.conn()?;
        let mut tx = conn.start_transaction(TxOpts::default())?;
            tx.exec_drop("DROP TABLE Calendar;",())?;
            tx.exec_drop("DROP TABLE Journal_task_tags;",())?;
            tx.exec_drop("DROP TABLE Journal_tasks;",())?;
            tx.exec_drop("DROP TABLE Ledger;",())?;
            tx.exec_drop("DROP TABLE Note_task_tags;",())?;
            tx.exec_drop("DROP TABLE Note_tasks;",())?;
            tx.exec_drop("DROP TABLE Todo_task_tags;",())?;
            tx.exec_drop("DROP TABLE Todo_tasks;",())?;
            tx.exec_drop("DROP TABLE tags;",())?;
        tx.commit()?;
        Ok(())
    }


    pub fn save_all(&self,features:&Feature_set) -> mysql::Result<()>{
        let mut conn = self.conn()?;
        let mut tx = conn.start_transaction(TxOpts::default())?;
    features.tags.iter().for_each(|x| {
        self.add_tag(&mut tx, x).unwrap();
    });
    
    tx.commit()?;

    features.calendars.iter().for_each(|x| {
        self.add_event(x).unwrap();
    });

    self.save_ledger(&features.finance).unwrap();

    features.journals.iter().for_each(|x| {
        self.save_journal_task(&x).unwrap();
    });

    features.notes.iter().for_each(|x| {
        self.save_note_task(&x).unwrap();
    });

    features.todos.iter().for_each(|x|{
        self.save_todo_task(&x).unwrap();
    });
        


        Ok(())
    }




}





}


