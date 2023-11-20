DROP TABLE IF EXISTS `consciences`;
CREATE TABLE `consciences` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nom` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

LOCK TABLES `consciences` WRITE;
INSERT INTO `consciences` VALUES (1,'Mgr Chaussé'),(2,'Eugène Claprood'),(3,'Gustave Désourdy'),(4,'Pierrette Tanguay'),(5,'flag cst{f1x_d3_c0n5c13nc35_c0rr0mpu35}');
UNLOCK TABLES;

CREATE USER 'sqli1'@'%' IDENTIFIED BY 'EjdCS55ttbnQRx26Qv';
GRANT SELECT ON `consciences` TO 'sqli1'@'%';
FLUSH PRIVILEGES;
