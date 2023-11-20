<?php
$servername = "db-bool";
$username = "root";
$password = "thepasswordistotallynotroot!";
$dbname = "mydb";

$conn = new mysqli($servername, $username, $password, $dbname);

if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}

if ($_SERVER["REQUEST_METHOD"] == "POST") {
    $username = $_POST["username"];
    $password = $_POST["password"];

    $sql = "SELECT * FROM users WHERE username = '$username' AND password = '$password'";
    $result = $conn->query($sql);

    if ($result->num_rows > 0) {
        echo "cst{f1rst_sql_b00l3an_1nj3ct10n}";
    } else {
        echo "Login failed. Please try again.";
    }
}

$conn->close();
?>
