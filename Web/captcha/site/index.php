<?php
session_start();

error_reporting(0);
ini_set('display_errors', 0);
ini_set('display_startup_errors', 0);

define('TOTAL', 20000);

$succes = false;
$captcha = "";
$flag = false;

$valeursPossibles = [1, 2, 3, 4];

// Traitement formulaire
if (isset($_POST['captcha'])) {
	$compteur = $_SESSION['compteur'] ?? 0;
    $captcha = $_POST['captcha'] ?? "captcha";
    $type = $_SESSION['type'] ?? -1;

    if (in_array($captcha, $valeursPossibles) && in_array($type, $valeursPossibles) && $type == $captcha) {
		$compteur++;
		$succes = true;

		if ($compteur >= TOTAL) {
			$flag = true;
        }
    }
}

// Reset
if (!$succes) {
    $compteur = 0;
}

$_SESSION['compteur'] = $compteur;
?>

<!doctype html>
<html lang="fr">
<head>
	<meta charset="utf-8">
	<title>Arpanet</title>
	<meta name="viewport" content="width=device-width, initial-scale=1">
    <link href="https://fonts.googleapis.com/css?family=Roboto:wght@400;900" rel="stylesheet">
    <link rel="stylesheet" href="css/app.css?v=1.0.0">
</head>
<body>
    <div class="container">
        <header>
            <h1>Connexion à l'Arpanet</h1>
        </header>

        <article>
            <div id="captcha">
                <img src="image.php">
            </div>

            <form action="index.php" method="post">
                <input type="radio" name="captcha" value="1"> Bleu&nbsp;&nbsp;
                <input type="radio" name="captcha" value="2"> Vert&nbsp;&nbsp;
                <input type="radio" name="captcha" value="3"> Rouge&nbsp;&nbsp;
                <input type="radio" name="captcha" value="4"> Jaune<br><br>

                <input type="submit" name="submit" value="VALIDER">
            </form>

            <?php if ($succes): ?>
            <div class='succes'>
                <?php if ($flag): ?>
                    <div id="flag">cst{4cc35_4u70r1533_v3r5_4rp4n37}</div>
                <?php else: ?>
	                <?= $compteur; ?> sur <?= TOTAL; ?>
                <?php endif; ?>
            </div>
            <?php else: ?>
                <?php if (!empty($captcha)): ?>
                    <div class='erreur'>ERREUR</div>
                <?php endif; ?>
            <?php endif; ?>
        </article>

        <footer>Université de Californie &copy;</footer>
    </div>
</body>