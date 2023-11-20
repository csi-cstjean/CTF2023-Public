async function login(username, password) {
    document.getElementById('errorContainer').innerHTML =
    document.cookie === "admin=true" ?
    await fetch('http://auth103nginx.ctf:10000/flag').then(res => res.text()) :
    "Wrong username or password !";
}
