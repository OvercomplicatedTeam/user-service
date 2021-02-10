CREATE TABLE parkings(
    parking_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    admin_id SERIAL NOT NULL ,
    CONSTRAINT fk_admin
        FOREIGN KEY(admin_id)
        REFERENCES users(user_id)
                     ON DELETE CASCADE
)