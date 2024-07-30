// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri;

window.addEventListener('load', function() {
    document.getElementById('form_login').addEventListener("submit", function(event) {
        // Prevent the default form submission
        event.preventDefault();
    
        // Get form data
        const formData = new FormData(document.getElementById('form_login'));
    
        // Convert form data to an object
        const data = {};
        formData.forEach((value, key) => {
            data[key] = value;
        });
        console.log(formData);
        console.log(data);
        invoke('logar_tauri', {nome: data.username, senha: data.password}).then((response) => {
            console.log(response);
            //alert('Bem vindo!');
            window.location.href = '/home/index.html';
        })
        .catch((e) => {
            console.log(e);
            alert('Login inválido!');
            limpar_form();
        })
    })
});

function limpar_form(){
    document.getElementById('form_login').reset();
}