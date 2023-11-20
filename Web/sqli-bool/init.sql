-- Créez une table "users" pour stocker les informations d'authentification
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(50) NOT NULL
);

-- Insérez quelques données factices
INSERT INTO users (username, password) VALUES
('admin', 'secret_password'),
('user1', 'password123');
