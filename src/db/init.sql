-- Users table
CREATE TABLE IF NOT EXISTS users (
    uid CHAR(36) PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255),
    displayed_name VARCHAR(255) NOT NULL,
    status ENUM('online', 'offline', 'away', 'busy') DEFAULT 'offline',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Peers table (peer connections)
CREATE TABLE IF NOT EXISTS peers (
    peer_id CHAR(36) PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    peer_name VARCHAR(255) NOT NULL,
    status ENUM('connected', 'disconnected', 'pending') DEFAULT 'disconnected',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_connected TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(uid) ON DELETE CASCADE
);

-- Peer connections/IP mappings
CREATE TABLE IF NOT EXISTS peer_connections (
    id INT AUTO_INCREMENT PRIMARY KEY,
    peer_id CHAR(36) NOT NULL,
    connection_string VARCHAR(512) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (peer_id) REFERENCES peers(peer_id) ON DELETE CASCADE
);

-- Chat messages table
CREATE TABLE IF NOT EXISTS messages (
    message_id CHAR(36) PRIMARY KEY,
    sender_id CHAR(36) NOT NULL,
    receiver_id CHAR(36) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    read_at TIMESTAMP NULL,
    FOREIGN KEY (sender_id) REFERENCES users(uid) ON DELETE CASCADE,
    FOREIGN KEY (receiver_id) REFERENCES users(uid) ON DELETE CASCADE
);

-- Calls/Sessions table
CREATE TABLE IF NOT EXISTS calls (
    call_id CHAR(36) PRIMARY KEY,
    initiated_by CHAR(36) NOT NULL,
    call_type ENUM('audio', 'video', 'text', 'audio_video', 'audio_text', 'video_text', 'all') NOT NULL,
    status ENUM('initiated', 'connected', 'ended', 'failed', 'rejected') DEFAULT 'initiated',
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP NULL,
    duration_seconds INT DEFAULT 0,
    is_success BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (initiated_by) REFERENCES users(uid) ON DELETE CASCADE
);

-- Call participants table (for multi-party calls)
CREATE TABLE IF NOT EXISTS call_participants (
    id INT AUTO_INCREMENT PRIMARY KEY,
    call_id CHAR(36) NOT NULL,
    user_id CHAR(36) NOT NULL,
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    left_at TIMESTAMP NULL,
    FOREIGN KEY (call_id) REFERENCES calls(call_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(uid) ON DELETE CASCADE,
    UNIQUE KEY unique_participant (call_id, user_id)
);

-- Call logs table (for historical records)
CREATE TABLE IF NOT EXISTS call_logs (
    log_id CHAR(36) PRIMARY KEY,
    call_id CHAR(36),
    comm_type_audio BOOLEAN DEFAULT FALSE,
    comm_type_video BOOLEAN DEFAULT FALSE,
    comm_type_text BOOLEAN DEFAULT FALSE,
    comm_type_screen BOOLEAN DEFAULT FALSE,
    duration_seconds INT DEFAULT 0,
    is_success BOOLEAN DEFAULT FALSE,
    hash VARCHAR(64),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (call_id) REFERENCES calls(call_id) ON DELETE SET NULL
);

-- Indexes for performance
CREATE INDEX idx_messages_sender ON messages(sender_id);
CREATE INDEX idx_messages_receiver ON messages(receiver_id);
CREATE INDEX idx_messages_created ON messages(created_at);
CREATE INDEX idx_peers_user ON peers(user_id);
CREATE INDEX idx_peers_status ON peers(status);
CREATE INDEX idx_calls_initiated_by ON calls(initiated_by);
CREATE INDEX idx_calls_status ON calls(status);
CREATE INDEX idx_calls_started_at ON calls(started_at);
CREATE INDEX idx_call_participants_call ON call_participants(call_id);
CREATE INDEX idx_call_participants_user ON call_participants(user_id);
