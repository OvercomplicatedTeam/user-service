CREATE TABLE parkings_consumers(
    parking_id SERIAL NOT NULL REFERENCES parkings(parking_id),
    consumer_id SERIAL NOT NULL REFERENCES users(user_id),
    PRIMARY KEY (parking_id, consumer_id)
)