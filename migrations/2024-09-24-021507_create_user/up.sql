CREATE TABLE IF NOT EXISTS "user" (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL ,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    username VARCHAR(50),
    email VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    status INT DEFAULT 1
);

COMMENT ON COLUMN "user".status IS '1: active, 0: inactive';
