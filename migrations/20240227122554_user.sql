CREATE TABLE app_user (
    id VARCHAR(64) DEFAULT uuid_generate_v4()::TEXT PRIMARY KEY,
    discord_id VARCHAR(64) UNIQUE,
    github_id VARCHAR(64) UNIQUE,
    email VARCHAR(64) UNIQUE,
    password_hash VARCHAR(128),
    username VARCHAR(39) UNIQUE NOT NULL,
    avatar_url VARCHAR(256) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
