CREATE TABLE IF NOT EXISTS "user" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL ,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    username VARCHAR(50),
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    ask_for_new_password INT DEFAULT 0,
    status INT DEFAULT 1
);

COMMENT ON COLUMN "user".status IS '1: active, 0: inactive';
COMMENT ON COLUMN "user".ask_for_new_password IS '1: need reset password, 0: not need reset password';
