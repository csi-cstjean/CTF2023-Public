function login(username, password) {
    document.getElementById('errorContainer').innerHTML = 
        username == "admin" && password == "admin" ?  
        "Congrats ! Here is your flag : cst{y0u_st4art3d_l34rn1ng!}" : 
        "Wrong username or password !";
}