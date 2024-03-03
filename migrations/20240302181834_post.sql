CREATE TABLE app_post (
    id VARCHAR(64) DEFAULT uuid_generate_v4()::TEXT PRIMARY KEY,
    user_id VARCHAR(64),
    content VARCHAR(512),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES app_user(id)
)
