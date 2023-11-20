<?php
    ini_set('display_errors', 0);
    ini_set('display_startup_errors', 0);
    error_reporting(0);
?>
<!doctype html>
<html lang="fr">
<head>
	<meta charset="utf-8">
	<title>Répertoire des consciences</title>
	<meta name="viewport" content="width=device-width, initial-scale=1">
    <link href="https://fonts.googleapis.com/css?family=RobotoR:wght@400;900" rel="stylesheet">
    <link rel="stylesheet" href="css/app.css?v=1.0.0">
</head>

<body>
    <div class="container">
        <header>
            <h1>Répertoire des consciences</h1>
        </header>

        <article>
            <form action="index.php" method="post">
                <input type="text" name="nom" placeholder="Nom de la conscience">
                <input type="submit" value="CHERCHER">
            </form>

            <div class="resultat">
                <?php
                if (isset($_POST["nom"])) {
                    $query_nom = $_POST["nom"];

                    try {
                        $db_servername = "sqli1-corrupted-db";
                        $db_username = "sqli1";
                        $db_password = "EjdCS55ttbnQRx26Qv";
                        $db_name = "database";

                        $conn = new mysqli($db_servername, $db_username, $db_password, $db_name);

                        if ($conn->connect_error) {
                            die("Erreur connexion BD : " . $conn->connect_error);
                        }

                        // Faille SQLi
                        $sql = "SELECT * FROM consciences WHERE nom LIKE '%" . $query_nom . "%'";

                        $result = $conn->query($sql);

                        if ($result->num_rows > 0) {
                            while ($data = mysqli_fetch_array($result)) {
                                $nom = $data['nom'];

                                if (str_contains($nom, 'flag')) {
                                    echo "<div class='erreur'>CORRUPTED DATA</div>";
                                } else {
                                    echo "<div class='nom'>{$nom}</div>";
                                }

                            }
                        } else {
                            echo "<div class='erreur'>Aucun résultat</div>";
                        }
                    } catch (mysqli_sql_exception $e) {
                        echo "<div class='erreur'>ERREUR</div>";
                    } finally {
                        if (isset($conn)) {
                            $conn->close();
                        }
                    }
                }
                ?>
            </div>
        </article>

        <footer>&copy; 1968</footer>
    </div>
</body>

</html>
