SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";
CREATE DATABASE IF NOT EXISTS `CoDi` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
USE `CoDi`;

DELIMITER $$
DROP PROCEDURE IF EXISTS `TransfererMemoire`$$
CREATE DEFINER=`root`@`localhost` PROCEDURE `TransfererMemoire` ()   BEGIN
    INSERT INTO backup (id, name, flag)
    SELECT id, name, flag FROM memory;

    DELETE FROM backup;
END$$

DROP PROCEDURE IF EXISTS `verifierTransfert`$$
CREATE DEFINER=`root`@`localhost` PROCEDURE `verifierTransfert` ()   BEGIN
    SELECT id, name, flag
    FROM backup;
END$$

DELIMITER ;

DROP TABLE IF EXISTS `backup`;
CREATE TABLE IF NOT EXISTS `backup` (
  `id` int(11) NOT NULL,
  `name` varchar(50) NOT NULL,
  `flag` varchar(50) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

DROP TABLE IF EXISTS `memory`;
CREATE TABLE IF NOT EXISTS `memory` (
  `id` int(11) NOT NULL,
  `name` varchar(50) NOT NULL,
  `flag` varchar(50) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `memory` (`id`, `name`, `flag`) VALUES
(1, 'Gustave Désourdy', 'cst{c0ncur3ncy_1s_d5ng3r0us}');
COMMIT;



CREATE USER IF NOT EXISTS 'user'@'%' IDENTIFIED BY 'password';
-- Make sure the user doesn't have unwanted additional privileges
REVOKE ALL PRIVILEGES ON `CoDi`.* FROM 'user'@'%'; 
REVOKE ALL PRIVILEGES ON *.* FROM 'user'@'%';

-- Grant execution and SHOW permissions on stored procedures
GRANT EXECUTE, SHOW VIEW ON `CoDi`.* TO `user`@`%`;
GRANT EXECUTE ON PROCEDURE `CoDi`.`verifiertransfert` TO `user`@`%`;
GRANT EXECUTE ON PROCEDURE `CoDi`.`transferermemoire` TO `user`@`%`;
-- Accorder la permission SELECT sur la table mysql.proc
GRANT SELECT ON mysql.proc TO `user`@`%`;
