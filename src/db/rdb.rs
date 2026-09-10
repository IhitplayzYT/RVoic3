pub mod rdb {
    use std::collections::HashMap;
    use uuid::Uuid;
    use chrono::{DateTime, TimeDelta, Utc};
    use mysql::{params, prelude::*, Pool, PooledConn, TxOpts};
    use serde::{Deserialize, Serialize};

    // ==================== ENUMS ====================

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum UserStatus {
        Online,
        Offline,
        Away,
        Busy,
    }

    impl std::fmt::Display for UserStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                UserStatus::Online => write!(f, "Online"),
                UserStatus::Offline => write!(f, "Offline"),
                UserStatus::Away => write!(f, "Away"),
                UserStatus::Busy => write!(f, "Busy"),
            }
        }
    }

    impl From<&str> for UserStatus {
        fn from(s: &str) -> Self {
            match s.to_lowercase().as_str() {
                "online" => UserStatus::Online,
                "offline" => UserStatus::Offline,
                "away" => UserStatus::Away,
                "busy" => UserStatus::Busy,
                _ => UserStatus::Offline,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PeerStatus {
        Connected,
        Disconnected,
        Pending,
    }

    impl std::fmt::Display for PeerStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PeerStatus::Connected => write!(f, "Connected"),
                PeerStatus::Disconnected => write!(f, "Disconnected"),
                PeerStatus::Pending => write!(f, "Pending"),
            }
        }
    }

    impl From<&str> for PeerStatus {
        fn from(s: &str) -> Self {
            match s.to_lowercase().as_str() {
                "connected" => PeerStatus::Connected,
                "disconnected" => PeerStatus::Disconnected,
                "pending" => PeerStatus::Pending,
                _ => PeerStatus::Disconnected,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CallType {
        Audio,
        Video,
        Text,
        AudioVideo,
        AudioText,
        VideoText,
        All,
    }

    impl std::fmt::Display for CallType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                CallType::Audio => write!(f, "Audio"),
                CallType::Video => write!(f, "Video"),
                CallType::Text => write!(f, "Text"),
                CallType::AudioVideo => write!(f, "Audio_video"),
                CallType::AudioText => write!(f, "Audio_text"),
                CallType::VideoText => write!(f, "Video_text"),
                CallType::All => write!(f, "All"),
            }
        }
    }

    impl From<&str> for CallType {
        fn from(s: &str) -> Self {
            match s.to_lowercase().as_str() {
                "audio" => CallType::Audio,
                "video" => CallType::Video,
                "text" => CallType::Text,
                "audio_video" => CallType::AudioVideo,
                "audio_text" => CallType::AudioText,
                "video_text" => CallType::VideoText,
                "all" => CallType::All,
                _ => CallType::Audio,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CallStatus {
        Initiated,
        Connected,
        Ended,
        Failed,
        Rejected,
    }

    impl std::fmt::Display for CallStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                CallStatus::Initiated => write!(f, "Initiated"),
                CallStatus::Connected => write!(f, "Connected"),
                CallStatus::Ended => write!(f, "Ended"),
                CallStatus::Failed => write!(f, "Failed"),
                CallStatus::Rejected => write!(f, "Rejected"),
            }
        }
    }

    impl From<&str> for CallStatus {
        fn from(s: &str) -> Self {
            match s.to_lowercase().as_str() {
                "initiated" => CallStatus::Initiated,
                "connected" => CallStatus::Connected,
                "ended" => CallStatus::Ended,
                "failed" => CallStatus::Failed,
                "rejected" => CallStatus::Rejected,
                _ => CallStatus::Initiated,
            }
        }
    }

    // ==================== STRUCTS ====================

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub uid: Uuid,
        pub username: String,
        pub password: Option<String>,
        pub displayed_name: String,
        pub status: UserStatus,
        pub created_at: DateTime<Utc>,
        pub last_seen: DateTime<Utc>,
    }

    impl User {
        pub fn new(username: String, password: Option<String>, displayed_name: Option<String>) -> Self {
            let now = Utc::now();
            Self {uid: Uuid::new_v4(),username: username.clone(),password,displayed_name: displayed_name.unwrap_or(username),status: UserStatus::Offline,created_at: now,last_seen: now}
        }

        pub fn anonymous() -> Self {
            let now = Utc::now();
            Self {uid: Uuid::new_v4(),username: "ANONYMOUS".to_string(),password: None,displayed_name: "ANONYMOUS".to_string(),status: UserStatus::Online,created_at: now,last_seen: now}
        }

    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Peer {
        pub peer_id: Uuid,
        pub peer_name: String,
        pub user_id: Uuid,
        pub status: PeerStatus,
        pub created_at: DateTime<Utc>,
        pub last_connected: Option<DateTime<Utc>>,
    }

    impl Peer {
        pub fn new(user_id: Uuid, peer_name: String) -> Self {
            let now = Utc::now();
            Self {peer_id: Uuid::new_v4(),user_id,peer_name,status: PeerStatus::Disconnected,created_at: now,last_connected: None}
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PeerConnection {
        pub id: Option<u64>,
        pub peer_id: Uuid,
        pub connection_string: String,
        pub created_at: DateTime<Utc>,
    }

    impl PeerConnection {
        pub fn new(peer_id: Uuid, connection_string: String) -> Self {
            Self {id: None,peer_id,connection_string,created_at: Utc::now()}
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Message {
        pub message_id: Uuid,
        pub sender_id: Uuid,
        pub receiver_id: Uuid,
        pub content: String,
        pub created_at: DateTime<Utc>,
        pub read_at: Option<DateTime<Utc>>,
    }

    impl Message {
        pub fn new(sender_id: Uuid, receiver_id: Uuid, content: String) -> Self {
            Self {message_id: Uuid::new_v4(),sender_id,receiver_id,content,created_at: Utc::now(),read_at: None}
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Call {
        pub call_id: Uuid,
        pub caller: Uuid,
        pub call_type: CallType,
        pub status: CallStatus,
        pub started_at: DateTime<Utc>,
        pub ended_at: Option<DateTime<Utc>>,
        pub duration_seconds: i32,
        pub is_success: bool,
    }

    impl Call {
        pub fn new(caller: Uuid, call_type: CallType) -> Self {
            let now = Utc::now();
            Self {call_id: Uuid::new_v4(),caller,call_type,status: CallStatus::Initiated,started_at: now,ended_at: None,duration_seconds: 0,is_success: false}
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CallParticipant {
        pub id: Option<u64>,
        pub call_id: Uuid,
        pub user_id: Uuid,
        pub joined_at: DateTime<Utc>,
        pub left_at: Option<DateTime<Utc>>,
    }

    impl CallParticipant {
        pub fn new(call_id: Uuid, user_id: Uuid) -> Self {
            Self {id: None,call_id,user_id,joined_at: Utc::now(),left_at: None}
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CallLog {
        pub log_id: Uuid,
        pub call_id: Option<Uuid>,
        pub comm_audio: bool,
        pub comm_video: bool,
        pub comm_text: bool,
        pub comm_screen: bool,
        pub duration_seconds: i32,
        pub is_success: bool,
        pub hash: String,
        pub created_at: DateTime<Utc>,
    }

    impl CallLog {
        pub fn new(call_id: Option<Uuid>) -> Self {
            Self {log_id: Uuid::new_v4(),call_id,comm_audio: false,comm_video: false,comm_text: false,comm_screen: false,duration_seconds: 0,is_success: false,hash: String::new(),created_at: Utc::now(),
            }
        }
    }

    // Helper struct for PeerConnection queries to avoid tuple size limits
    #[derive(Debug, Clone)]
    struct PeerConnectionRow {
        id: u64,
        peer_id: String,
        connection_string: String,
        created_at: DateTime<Utc>,
    }


    impl FromRow for PeerConnectionRow {

        fn from_row(row: mysql::Row) -> Self
        where
            Self: Sized,
        {
            Self {id: row.get("id").unwrap(),peer_id: row.get("peer_id").unwrap(),connection_string: row.get("connection_string").unwrap(),created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc)}
        }

        fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
        where
            Self: Sized
        {
            Ok(Self {id: row.get("id").unwrap(),peer_id: row.get("peer_id").unwrap(),connection_string: row.get("connection_string").unwrap(),created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc)})
        }
    }

    #[derive(Debug, Clone)]
    struct MessageRow {
        message_id: String,
        sender_id: String,
        receiver_id: String,
        content: String,
        created_at: DateTime<Utc>,
        read_at: Option<DateTime<Utc>>,
    }

    impl FromRow for MessageRow {

        fn from_row(row: mysql::Row) -> Self
        where
            Self: Sized,
        {
            
            Self {message_id: row.get("message_id").unwrap(),sender_id: row.get("sender_id").unwrap(),receiver_id: row.get("receiver_id").unwrap(),content: row.get("content").unwrap(),created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc),read_at: row.get::<Option<String>, _>("read_at").flatten().map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc))}
        }

        fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
        where
            Self: Sized
        {
            Ok(Self {message_id: row.get("message_id").unwrap(),sender_id: row.get("sender_id").unwrap(),receiver_id: row.get("receiver_id").unwrap(),content: row.get("content").unwrap(),created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc),read_at: row.get::<Option<String>, _>("read_at").flatten().map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc))})
        }
    }

    // Helper struct for Call queries to avoid tuple size limits
    #[derive(Debug, Clone)]
    struct CallRow {
        call_id: String,
        caller: String,
        call_type: String,
        status: String,
        started_at: DateTime<Utc>,
        ended_at: Option<DateTime<Utc>>,
        duration_seconds: i32,
        is_success: bool,
    }

    impl FromRow for CallRow {
       fn from_row(row: mysql::Row) -> Self
       where
           Self: Sized,
       {
            Self {call_id: row.get("call_id").unwrap(),caller: row.get("caller").unwrap(),call_type: row.get("call_type").unwrap(),status: row.get("status").unwrap(),started_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("started_at").unwrap()).unwrap().with_timezone(&Utc),ended_at: row.get::<Option<String>, _>("ended_at").flatten().map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),duration_seconds: row.get("duration_seconds").unwrap(),is_success: row.get("is_success").unwrap()}
       } 

        fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
        where
            Self: Sized
        {
            Ok(Self {call_id: row.get("call_id").unwrap(),caller: row.get("caller").unwrap(),call_type: row.get("call_type").unwrap(),status: row.get("status").unwrap(),started_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("started_at").unwrap()).unwrap().with_timezone(&Utc),ended_at: row.get::<Option<String>, _>("ended_at").flatten().map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),duration_seconds: row.get("duration_seconds").unwrap(),is_success: row.get("is_success").unwrap()})
        }
    }

    // Helper struct for CallParticipant queries to avoid tuple size limits
    #[derive(Debug, Clone)]
    struct CallParticipantRow {
        id: u64,
        call_id: String,
        user_id: String,
        joined_at: DateTime<Utc>,
        left_at: Option<DateTime<Utc>>,
    }

    impl FromRow for CallParticipantRow {
        
        fn from_row(row: mysql::Row) -> Self
        where
            Self: Sized,
        {
            Self {id: row.get("id").unwrap(),call_id: row.get("call_id").unwrap(),user_id: row.get("user_id").unwrap(),joined_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("joined_at").unwrap()).unwrap().with_timezone(&Utc),left_at: row.get::<Option<String>, _>("left_at").flatten().map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc))}
        }
        

        fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
        where
            Self: Sized
        {
            Ok(Self {id: row.get("id").unwrap(),call_id: row.get("call_id").unwrap(),user_id: row.get("user_id").unwrap(),joined_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("joined_at").unwrap()).unwrap().with_timezone(&Utc),left_at: row.get::<Option<String>, _>("left_at").flatten().map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc))})
        }
    }

    #[derive(Debug, Clone)]
    struct CallLogRow {
        log_id: String,
        call_id: Option<String>,
        comm_type_audio: bool,
        comm_type_video: bool,
        comm_type_text: bool,
        comm_type_screen: bool,
        duration_seconds: i32,
        is_success: bool,
        hash: String,
        created_at: DateTime<Utc>,
    }

    impl FromRow for CallLogRow {

        fn from_row(row: mysql::Row) -> Self
        where
            Self: Sized,
        {
            Self {log_id: row.get("log_id").unwrap(),call_id: row.get("call_id"),comm_type_audio: row.get("comm_type_audio").unwrap(),comm_type_video: row.get("comm_type_video").unwrap(),comm_type_text: row.get("comm_type_text").unwrap(),comm_type_screen: row.get("comm_type_screen").unwrap(),duration_seconds: row.get("duration_seconds").unwrap(),is_success: row.get("is_success").unwrap(),hash: row.get("hash").unwrap(),created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc)}
        }

        fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
        where
            Self: Sized
        {
            Ok(Self {log_id: row.get("log_id").unwrap(),call_id: row.get("call_id"),comm_type_audio: row.get("comm_type_audio").unwrap(),comm_type_video: row.get("comm_type_video").unwrap(),comm_type_text: row.get("comm_type_text").unwrap(),comm_type_screen: row.get("comm_type_screen").unwrap(),duration_seconds: row.get("duration_seconds").unwrap(),is_success: row.get("is_success").unwrap(),hash: row.get("hash").unwrap(),created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc)})
        }
    }

    // Helper struct for User queries to avoid tuple size limits
    #[derive(Debug, Clone)]
    struct UserRow {
        uid: String,
        username: String,
        password: Option<String>,
        displayed_name: String,
        status: String,
        created_at: DateTime<Utc>,
        last_seen: DateTime<Utc>,
    }

    impl FromRow for UserRow {
        fn from_row(row: mysql::Row) -> Self
        where
            Self: Sized,
        {
            Self {
                uid: row.get("uid").unwrap(),
                username: row.get("username").unwrap(),
                password: row.get("password"),
                displayed_name: row.get("displayed_name").unwrap(),
                status: row.get("status").unwrap(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc),
                last_seen: DateTime::parse_from_rfc3339(&row.get::<String, _>("last_seen").unwrap()).unwrap().with_timezone(&Utc),
            }
        }

        fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
        where
            Self: Sized
        {
            Ok(Self {
                uid: row.get("uid").unwrap(),
                username: row.get("username").unwrap(),
                password: row.get("password"),
                displayed_name: row.get("displayed_name").unwrap(),
                status: row.get("status").unwrap(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc),
                last_seen: DateTime::parse_from_rfc3339(&row.get::<String, _>("last_seen").unwrap()).unwrap().with_timezone(&Utc),
            })
        }
    }

    // Helper struct for Peer queries to avoid tuple size limits
    #[derive(Debug, Clone)]
    struct PeerRow {
        peer_id: String,
        user_id: String,
        peer_name: String,
        status: String,
        created_at: DateTime<Utc>,
        last_connected: Option<DateTime<Utc>>,
    }

    impl FromRow for PeerRow {
        fn from_row(row: mysql::Row) -> Self
        where
            Self: Sized,
        {
            Self {
                peer_id: row.get("peer_id").unwrap(),
                user_id: row.get("user_id").unwrap(),
                peer_name: row.get("peer_name").unwrap(),
                status: row.get("status").unwrap(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc),
                last_connected: row.get::<Option<String>, _>("last_connected").flatten().map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
            }
        }

        fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
        where
            Self: Sized
        {
            Ok(Self {
                peer_id: row.get("peer_id").unwrap(),
                user_id: row.get("user_id").unwrap(),
                peer_name: row.get("peer_name").unwrap(),
                status: row.get("status").unwrap(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at").unwrap()).unwrap().with_timezone(&Utc),
                last_connected: row.get::<Option<String>, _>("last_connected").flatten().map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
            })
        }
    }

    pub struct Database {
        pool: Pool,
    }

    impl Database {
        // ------------------- INIT APIs -------------------
        pub fn new(url: &str) -> mysql::Result<Self> {
            Ok(Self { pool: Pool::new(url)? })
        }

        fn conn(&self) -> mysql::Result<PooledConn> {
            self.pool.get_conn()
        }

        pub fn init_dbs(&self) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            let mut tx = conn.start_transaction(TxOpts::default())?;
            let sql_path = "src/db/init.sql";
            let sql_content = std::fs::read_to_string(sql_path)?;
            for statement in sql_content.split(';') {
                let stmt = statement.trim();
                if !stmt.is_empty() {
                    tx.exec_drop(stmt, ())?;
                }
            }
            tx.commit()?;
            Ok(())
        }

        pub fn clear(&self) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            let mut tx = conn.start_transaction(TxOpts::default())?;
            tx.exec_drop("DROP TABLE IF EXISTS call_logs", ())?;
            tx.exec_drop("DROP TABLE IF EXISTS call_participants", ())?;
            tx.exec_drop("DROP TABLE IF EXISTS calls", ())?;
            tx.exec_drop("DROP TABLE IF EXISTS messages", ())?;
            tx.exec_drop("DROP TABLE IF EXISTS peer_connections", ())?;
            tx.exec_drop("DROP TABLE IF EXISTS peers", ())?;
            tx.exec_drop("DROP TABLE IF EXISTS users", ())?;
            tx.commit()?;
            Ok(())
        }

        // ------------------- USER CRUD -------------------
        pub fn create_user(&self, user: &User) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "INSERT INTO users (uid, username, password, displayed_name, status, created_at, last_seen) VALUES (?, ?, ?, ?, ?, ?, ?)",
                (user.uid.to_string(), &user.username, &user.password, &user.displayed_name, user.status.to_string(), user.created_at.to_string(), user.last_seen.to_string()),
            )?;
            Ok(())
        }

        pub fn get_user(&self, uid: &Uuid) -> mysql::Result<Option<User>> {
            let mut conn = self.conn()?;
            let result: Option<UserRow> = conn.exec_first(
                "SELECT uid, username, password, displayed_name, status, created_at, last_seen FROM users WHERE uid = ?",
                (uid.to_string(),),
            )?;

            match result {
                Some(row) => {
                    Ok(Some(User {uid: Uuid::parse_str(&row.uid).unwrap(),username: row.username,password: row.password,displayed_name: row.displayed_name,status: UserStatus::from(row.status.as_str()),created_at: row.created_at,last_seen: row.last_seen}))
                }
                None => Ok(None),
            }
        }

        pub fn get_user_by_username(&self, username: &str) -> mysql::Result<Option<User>> {
            let mut conn = self.conn()?;
            let result: Option<UserRow> = conn.exec_first(
                "SELECT uid, username, password, displayed_name, status, created_at, last_seen FROM users WHERE username = ?",
                (username,),
            )?;

            match result {
                Some(row) => {
                    Ok(Some(User {uid: Uuid::parse_str(&row.uid).unwrap(),username: row.username,password: row.password,displayed_name: row.displayed_name,status: UserStatus::from(row.status.as_str()),created_at: row.created_at,last_seen: row.last_seen}))
                }
                None => Ok(None),
            }
        }

        pub fn update_user(&self, user: &User) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "UPDATE users SET username = ?, password = ?, displayed_name = ?, status = ?, last_seen = ? WHERE uid = ?",
                (&user.username, &user.password, &user.displayed_name, user.status.to_string(), user.last_seen.to_string(), user.uid.to_string()),
            )?;
            Ok(())
        }

        pub fn update_user_status(&self, uid: &Uuid, status: UserStatus) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "UPDATE users SET status = ?, last_seen = NOW() WHERE uid = ?",
                (status.to_string(), uid.to_string()),
            )?;
            Ok(())
        }

        pub fn delete_user(&self, uid: &Uuid) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop("DELETE FROM users WHERE uid = ?", (uid.to_string(),))?;
            Ok(())
        }

        pub fn list_users(&self) -> mysql::Result<Vec<User>> {
            let mut conn = self.conn()?;
            let results: Vec<UserRow> = conn.query(
                "SELECT uid, username, password, displayed_name, status, created_at, last_seen FROM users",
            )?;

            results
                .into_iter()
                .map(|row| {
                    Ok(User {uid: Uuid::parse_str(&row.uid).unwrap(),username: row.username,password: row.password,displayed_name: row.displayed_name,status: UserStatus::from(row.status.as_str()),created_at: row.created_at,last_seen: row.last_seen})
                }).collect()
        }

        // ------------------- PEER CRUD -------------------
        pub fn create_peer(&self, peer: &Peer) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "INSERT INTO peers (peer_id, user_id, peer_name, status, created_at, last_connected) VALUES (?, ?, ?, ?, ?, ?)",
                (peer.peer_id.to_string(), peer.user_id.to_string(), &peer.peer_name, peer.status.to_string(), peer.created_at.to_string(), peer.last_connected.map(|dt| dt.to_string())),
            )?;
            Ok(())
        }

        pub fn get_peer(&self, peer_id: &Uuid) -> mysql::Result<Option<Peer>> {
            let mut conn = self.conn()?;
            let result: Option<PeerRow> = conn.exec_first(
                "SELECT peer_id, user_id, peer_name, status, created_at, last_connected FROM peers WHERE peer_id = ?",
                (peer_id.to_string(),),
            )?;

            match result {
                Some(row) => {
                    Ok(Some(Peer {peer_id: Uuid::parse_str(&row.peer_id).unwrap(),user_id: Uuid::parse_str(&row.user_id).unwrap(),peer_name: row.peer_name,status: PeerStatus::from(row.status.as_str()),created_at: row.created_at,last_connected: row.last_connected}))
                }
                None => Ok(None),
            }
        }

        pub fn get_peers_by_user(&self, user_id: &Uuid) -> mysql::Result<Vec<Peer>> {
            let mut conn = self.conn()?;
            let results: Vec<PeerRow> = conn.exec(
                "SELECT peer_id, user_id, peer_name, status, created_at, last_connected FROM peers WHERE user_id = ?",
                (user_id.to_string(),),
            )?;

            results.into_iter().map(|row| {
                Ok(Peer {peer_id: Uuid::parse_str(&row.peer_id).unwrap(),user_id: Uuid::parse_str(&row.user_id).unwrap(),peer_name: row.peer_name,status: PeerStatus::from(row.status.as_str()),created_at: row.created_at,last_connected: row.last_connected,})
            }).collect()
        }

        pub fn update_peer(&self, peer: &Peer) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "UPDATE peers SET peer_name = ?, status = ?, last_connected = ? WHERE peer_id = ?",
                (&peer.peer_name, peer.status.to_string(), peer.last_connected.map(|dt| dt.to_string()), peer.peer_id.to_string()),
            )?;
            Ok(())
        }

        pub fn update_peer_status(&self, peer_id: &Uuid, status: PeerStatus) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            let last_connected = if status == PeerStatus::Connected { Some(Utc::now()) } else { None };
            conn.exec_drop(
                "UPDATE peers SET status = ?, last_connected = ? WHERE peer_id = ?",
                (status.to_string(), last_connected.map(|dt| dt.to_string()), peer_id.to_string()),
            )?;
            Ok(())
        }

        pub fn delete_peer(&self, peer_id: &Uuid) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop("DELETE FROM peers WHERE peer_id = ?", (peer_id.to_string(),))?;
            Ok(())
        }

        // ------------------- PEER CONNECTION CRUD -------------------
        pub fn add_peer_connection(&self, conn_data: &PeerConnection) -> mysql::Result<u64> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "INSERT INTO peer_connections (peer_id, connection_string, created_at) VALUES (?, ?, ?)",
                (conn_data.peer_id.to_string(), &conn_data.connection_string, conn_data.created_at.to_string()),
            )?;
            Ok(conn.last_insert_id())
        }

        pub fn get_peer_connections(&self, peer_id: &Uuid) -> mysql::Result<Vec<PeerConnection>> {
            let mut conn = self.conn()?;
            let results: Vec<PeerConnectionRow> = conn.exec(
                "SELECT id, peer_id, connection_string, created_at FROM peer_connections WHERE peer_id = ?",
                (peer_id.to_string(),),
            )?;

            results.into_iter().map(|row| {
                Ok(PeerConnection {id: Some(row.id),peer_id: Uuid::parse_str(&row.peer_id).unwrap(),connection_string: row.connection_string,created_at: row.created_at})
            }).collect()
        }

        pub fn delete_peer_connections(&self, peer_id: &Uuid) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop("DELETE FROM peer_connections WHERE peer_id = ?", (peer_id.to_string(),))?;
            Ok(())
        }

        // ------------------- MESSAGE CRUD -------------------
        pub fn create_message(&self, message: &Message) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "INSERT INTO messages (message_id, sender_id, receiver_id, content, created_at, read_at) VALUES (?, ?, ?, ?, ?, ?)",
                (message.message_id.to_string(), message.sender_id.to_string(), message.receiver_id.to_string(), &message.content, message.created_at.to_string(), message.read_at.map(|dt| dt.to_string())),
            )?;
            Ok(())
        }

        pub fn get_message(&self, message_id: &Uuid) -> mysql::Result<Option<Message>> {
            let mut conn = self.conn()?;
            let result: Option<MessageRow> = conn.exec_first(
                "SELECT message_id, sender_id, receiver_id, content, created_at, read_at FROM messages WHERE message_id = ?",
                (message_id.to_string(),),
            )?;

            match result {
                Some(row) => {
                    Ok(Some(Message {message_id: Uuid::parse_str(&row.message_id).unwrap(),sender_id: Uuid::parse_str(&row.sender_id).unwrap(),receiver_id: Uuid::parse_str(&row.receiver_id).unwrap(),content: row.content,created_at: row.created_at,read_at: row.read_at}))
                }
                None => Ok(None),
            }
        }

        pub fn get_messages_between_users(&self, user1: &Uuid, user2: &Uuid, limit: Option<u64>) -> mysql::Result<Vec<Message>> {
            let mut conn = self.conn()?;
            let (query, params) = if let Some(l) = limit {
                (format!("SELECT message_id, sender_id, receiver_id, content, created_at, read_at FROM messages WHERE (sender_id = ? AND receiver_id = ?) OR (sender_id = ? AND receiver_id = ?) ORDER BY created_at DESC LIMIT {}",l),
                 (user1.to_string(), user2.to_string(), user2.to_string(), user1.to_string()))
            } else {
                ("SELECT message_id, sender_id, receiver_id, content, created_at, read_at FROM messages WHERE (sender_id = ? AND receiver_id = ?) OR (sender_id = ? AND receiver_id = ?) ORDER BY created_at DESC".to_string(), 
                 (user1.to_string(), user2.to_string(), user2.to_string(), user1.to_string()))
            };
            
            let results: Vec<MessageRow> = conn.exec(&query, params)?;
            results.into_iter().map(|row| {
                    Ok(Message {message_id: Uuid::parse_str(&row.message_id).unwrap(),sender_id: Uuid::parse_str(&row.sender_id).unwrap(),receiver_id: Uuid::parse_str(&row.receiver_id).unwrap(),content: row.content,created_at: row.created_at,read_at: row.read_at})
                }).collect()
        }

        pub fn mark_message_read(&self, message_id: &Uuid) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "UPDATE messages SET read_at = NOW() WHERE message_id = ?",
                (message_id.to_string(),),
            )?;
            Ok(())
        }

        pub fn delete_message(&self, message_id: &Uuid) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop("DELETE FROM messages WHERE message_id = ?", (message_id.to_string(),))?;
            Ok(())
        }

        // ------------------- CALL CRUD -------------------
        pub fn create_call(&self, call: &Call) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "INSERT INTO calls (call_id, caller, call_type, status, started_at, ended_at, duration_seconds, is_success) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                (call.call_id.to_string(), call.caller.to_string(), call.call_type.to_string(), call.status.to_string(), call.started_at.to_string(), call.ended_at.map(|dt| dt.to_string()), call.duration_seconds, call.is_success),
            )?;
            Ok(())
        }

        pub fn get_call(&self, call_id: &Uuid) -> mysql::Result<Option<Call>> {
            let mut conn = self.conn()?;
            let result: Option<CallRow> = conn.exec_first(
                "SELECT call_id, caller, call_type, status, started_at, ended_at, duration_seconds, is_success FROM calls WHERE call_id = ?",
                (call_id.to_string(),),
            )?;

            match result {
                Some(row) => {
                    Ok(Some(Call {call_id: Uuid::parse_str(&row.call_id).unwrap(),caller: Uuid::parse_str(&row.caller).unwrap(),call_type: CallType::from(row.call_type.as_str()),status: CallStatus::from(row.status.as_str()),started_at: row.started_at,ended_at: row.ended_at,duration_seconds: row.duration_seconds,is_success: row.is_success}))
                }
                None => Ok(None),
            }
        }

        pub fn update_call(&self, call: &Call) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "UPDATE calls SET status = ?, ended_at = ?, duration_seconds = ?, is_success = ? WHERE call_id = ?",
                (call.status.to_string(), call.ended_at.map(|dt| dt.to_string()), call.duration_seconds, call.is_success, call.call_id.to_string()),
            )?;
            Ok(())
        }

        pub fn end_call(&self, call_id: &Uuid, is_success: bool) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            let ended_at = Utc::now();
            conn.exec_drop(
                "UPDATE calls SET status = 'ended', ended_at = ?, is_success = ? WHERE call_id = ?",
                (ended_at.to_string(), is_success, call_id.to_string()),
            )?;
            Ok(())
        }

        pub fn get_calls_by_user(&self, user_id: &Uuid) -> mysql::Result<Vec<Call>> {
            let mut conn = self.conn()?;
            let results: Vec<CallRow> = conn.exec(
                "SELECT call_id, caller, call_type, status, started_at, ended_at, duration_seconds, is_success FROM calls WHERE caller = ? ORDER BY started_at DESC",
                (user_id.to_string(),),
            )?;

            results.into_iter().map(|row| {
                    Ok(Call {call_id: Uuid::parse_str(&row.call_id).unwrap(),caller: Uuid::parse_str(&row.caller).unwrap(),call_type: CallType::from(row.call_type.as_str()),status: CallStatus::from(row.status.as_str()),started_at: row.started_at,ended_at: row.ended_at,duration_seconds: row.duration_seconds,is_success: row.is_success})
                }).collect()
        }

        pub fn delete_call(&self, call_id: &Uuid) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop("DELETE FROM calls WHERE call_id = ?", (call_id.to_string(),))?;
            Ok(())
        }

        // ------------------- CALL PARTICIPANT CRUD -------------------
        pub fn add_call_participant(&self, participant: &CallParticipant) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "INSERT INTO call_participants (call_id, user_id, joined_at, left_at) VALUES (?, ?, ?, ?)",
                (participant.call_id.to_string(), participant.user_id.to_string(), participant.joined_at.to_string(), participant.left_at.map(|dt| dt.to_string())),
            )?;
            Ok(())
        }

        pub fn get_call_participants(&self, call_id: &Uuid) -> mysql::Result<Vec<CallParticipant>> {
            let mut conn = self.conn()?;
            let results: Vec<CallParticipantRow> = conn.exec(
                "SELECT id, call_id, user_id, joined_at, left_at FROM call_participants WHERE call_id = ?",
                (call_id.to_string(),),
            )?;

            results.into_iter().map(|row| {
                    Ok(CallParticipant {id: Some(row.id),call_id: Uuid::parse_str(&row.call_id).unwrap(),user_id: Uuid::parse_str(&row.user_id).unwrap(),joined_at: row.joined_at,left_at: row.left_at,})
                }).collect()
        }

        pub fn remove_call_participant(&self, call_id: &Uuid, user_id: &Uuid) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "UPDATE call_participants SET left_at = NOW() WHERE call_id = ? AND user_id = ?",
                (call_id.to_string(), user_id.to_string()),
            )?;
            Ok(())
        }

        // ------------------- CALL LOG CRUD -------------------
        pub fn create_call_log(&self, log: &CallLog) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop(
                "INSERT INTO call_logs (log_id, call_id, comm_type_audio, comm_type_video, comm_type_text, comm_type_screen, duration_seconds, is_success, hash, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                (log.log_id.to_string(), log.call_id.map(|id| id.to_string()), log.comm_audio, log.comm_video, log.comm_text, log.comm_screen, log.duration_seconds, log.is_success, &log.hash, log.created_at.to_string()),
            )?;
            Ok(())
        }

        pub fn get_call_log(&self, log_id: &Uuid) -> mysql::Result<Option<CallLog>> {
            let mut conn = self.conn()?;
            let result: Option<CallLogRow> = conn.exec_first(
                "SELECT log_id, call_id, comm_type_audio, comm_type_video, comm_type_text, comm_type_screen, duration_seconds, is_success, hash, created_at FROM call_logs WHERE log_id = ?",
                (log_id.to_string(),),
            )?;

            match result {
                Some(row) => {
                    Ok(Some(CallLog {log_id: Uuid::parse_str(&row.log_id).unwrap(),call_id: row.call_id.and_then(|s| Uuid::parse_str(&s).ok()),comm_audio: row.comm_type_audio,comm_video: row.comm_type_video,comm_text: row.comm_type_text,comm_screen: row.comm_type_screen,duration_seconds: row.duration_seconds,is_success: row.is_success,hash: row.hash,created_at: row.created_at}))
                }
                None => Ok(None),
            }
        }

        pub fn get_call_logs_by_call(&self, call_id: &Uuid) -> mysql::Result<Vec<CallLog>> {
            let mut conn = self.conn()?;
            let results: Vec<CallLogRow> = conn.exec(
                "SELECT log_id, call_id, comm_type_audio, comm_type_video, comm_type_text, comm_type_screen, duration_seconds, is_success, hash, created_at FROM call_logs WHERE call_id = ?",
                (call_id.to_string(),),
            )?;

            results.into_iter().map(|row| {
                Ok(CallLog {log_id: Uuid::parse_str(&row.log_id).unwrap(),call_id: row.call_id.and_then(|s| Uuid::parse_str(&s).ok()),comm_audio: row.comm_type_audio,comm_video: row.comm_type_video,comm_text: row.comm_type_text,comm_screen: row.comm_type_screen,duration_seconds: row.duration_seconds,is_success: row.is_success,hash: row.hash,created_at: row.created_at,})
            }).collect()
        }

        pub fn delete_call_log(&self, log_id: &Uuid) -> mysql::Result<()> {
            let mut conn = self.conn()?;
            conn.exec_drop("DELETE FROM call_logs WHERE log_id = ?", (log_id.to_string(),))?;
            Ok(())
        }
    }
}
