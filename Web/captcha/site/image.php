<?php
    session_start();

    header("Content-type: image/png");
    $image = imagecreatefrompng('image/captcha.png');

    $x = [
        127,
        343,
        555,
        772
    ];

    $colors = [
        imagecolorallocate($image, 0, 0, 255),
        imagecolorallocate($image, 0, 255, 0),
        imagecolorallocate($image, 255, 0, 0),
        imagecolorallocate($image, 255, 255, 0)
    ];

    $rand = rand(0, 3);

    $_SESSION['type'] = $rand + 1;

    $ellipseColor = $colors[$rand];

    imagefilledellipse($image, $x[$rand], 98, 22, 22, $ellipseColor);

    imagesavealpha($image, true);
    imagepng($image);