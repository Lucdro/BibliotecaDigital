// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri;

const lista_pessoas = document.getElementById('lista_pessoas');

window.addEventListener('load', async function() {
    document.getElementById('filters_form').addEventListener("submit", async function(event) {
        // Prevent the default form submission
        event.preventDefault();
    
        // Get form data
        const formData = new FormData(document.getElementById('filters_form'));
    
        // Convert form data to an object
        const data = convertFormDataJson(formData);

        await buscarPessoas(data.nome);
    })
    document.getElementById('create_new').addEventListener("click", function(event){
        window.location.href = './form.html';
    })

    await buscarPessoas('');
});

function convertFormDataJson(data){
    const json = {};
    data.forEach((value, key) => {
        json[key] = value;
    });
    return json;
}

async function buscarPessoas(filtro){
    try{
        const response = await invoke('listar_pessoas_tauri', {nome: filtro});
        console.log(filtro)
        const pessoas = JSON.parse(response);
        //console.log(pessoas)
        lista_pessoas.innerHTML = '';
        for(const pessoa of pessoas){
            criarElementoPessoa(pessoa);
        }
    }catch(e){console.log(e);}
}

async function criarElementoPessoa(pessoa){
    lista_pessoas.innerHTML += `
        <div class="item_pessoa">
            <div>
                <span class="nome">${pessoa.nome}</span>
                <span class="cpf">${formatCPF(pessoa.cpf)}</span>
            </div>
            <div>
                <button class="alterar" onclick="alterarPessoa('${pessoa.cpf}')">Alterar</button>
                <button class="apagar" onclick="apagarPessoa('${pessoa.cpf}');">Apagar</button> 
            </div>
        </div>
    `
}

function alterarPessoa(cpf){
    window.location.href = './form.html?cpf='+cpf;
}

async function apagarPessoa(cpf){
    if(! await confirm("Deseja apagar a Pessoa?")){ return; }

    //Apagar Pessoa
    try{
        await invoke('apagar_pessoa_tauri' , {cpf: cpf});
    }catch(e){ console.log(e); }
    location.reload();
}

function formatCPF(value) {
    value = value.replace(/\D/g, ''); // Remove non-numeric characters
    value = value.replace(/(\d{3})(\d)/, '$1.$2');
    value = value.replace(/(\d{3})(\d)/, '$1.$2');
    value = value.replace(/(\d{3})(\d{1,2})$/, '$1-$2');
    return value;
}