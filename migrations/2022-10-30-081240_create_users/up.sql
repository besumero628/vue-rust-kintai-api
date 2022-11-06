CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    family_name VARCHAR(64) NOT NULL,
    given_name VARCHAR(64) NOT NULL,
    email VARCHAR(64) NOT NULL,
    password VARCHAR(64) NOT NULL,
    organization_id BIGINT UNSIGNED,

    CONSTRAINT fk_organization_id
      FOREIGN KEY (organization_id) 
      REFERENCES organizations (id)
      ON DELETE SET NULL ON UPDATE CASCADE,
      
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)