CREATE TABLE app_post (
    id VARCHAR(64) DEFAULT uuid_generate_v4()::TEXT PRIMARY KEY,
    user_id VARCHAR(64) NOT NULL,
    content VARCHAR(512) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    FOREIGN KEY (user_id) REFERENCES app_user(id)
)
