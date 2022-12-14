CREATE TABLE works
(
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    stamp_type_id INT NOT NULL,
    stamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_user_id
      FOREIGN KEY (user_id) 
      REFERENCES users (id)
      ON DELETE CASCADE ON UPDATE CASCADE,
    
    CONSTRAINT fk_stamp_type_id
      FOREIGN KEY (stamp_type_id) 
      REFERENCES stamp_types (id)
      ON UPDATE CASCADE,

    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)