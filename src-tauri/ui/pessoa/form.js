const { invoke } = window.__TAURI__.tauri;
const urlParams = new URLSearchParams(window.location.search);

const form = document.getElementById('pessoa_form');
const cpfInput = document.getElementById('cpf');
const nomeInput = document.getElementById('nome');
const emailInput = document.getElementById('email');
const celularInput = document.getElementById('celular');

let pessoa = undefined;

window.addEventListener('load', async (e) => {
    const cpf = urlParams.get('cpf');
    if(cpf){
        try{
            const response = await invoke('listar_pessoas_tauri', {cpf: cpf});
            const lista_cpf = JSON.parse(response);
            if(lista_cpf.length > 0) pessoa = lista_cpf[0];
            await preencherForm();
        }catch(e){console.log(e);}
    }
    form.addEventListener('submit', async (submit) => {
        submit.preventDefault();
        await salvarPessoa();
    })

    cpfInput.addEventListener('input', function(e) {
        let value = e.target.value;
        e.target.value = formatCPF(value);
    });

    celularInput.addEventListener('input', function(e) {
        let value = e.target.value;
        e.target.value = formatCelular(value);
    });
})

async function salvarPessoa(){
    const data = convertFormDataJson(new FormData(document.getElementById('pessoa_form')));
    data.cpt = data.cpf.replace(/\D/g, ''); //Limpa o cpf
    console.log(data);

    if(!validaData(data)){
        if(!validateCPF(data.cpf)) await alert('CPF inválido!');
        else await alert('Preencha CPF e nome para salvar');
        return;
    }

    novaPessoa = {
        cpf: data.cpf,
        nome: data.nome,
        email: data.email,
        celular: data.celular,
    }

    if(pessoa){
        try{
            const response = await invoke('alterar_pessoa_tauri',novaPessoa);
            pessoa = JSON.parse(response);
        }catch(e){console.log(e);}
    }else{
        try{
            const response = await invoke('criar_pessoa_tauri',novaPessoa);
            pessoa = JSON.parse(response);
        }catch(e){
            if(e.includes('pessoas_pkey')){
                alert('CPF já esta sendo usado por outra pessoa!');
            }
            console.log(e);
        }
    }

    window.location.href = `./index.html?cpf=${pessoa.cpf}`;
}

function validaData(data){
    return data.cpf 
    && validateCPF(data.cpf)
    && data.nome
    && data.nome.length > 0
}

async function preencherForm() {    
    cpfInput.value = pessoa.cpf ? formatCPF(pessoa.cpf) : '';
    nomeInput.value = pessoa.nome || '';
    emailInput.value = pessoa.email || '';
    celularInput.value = pessoa.celular || '';
}

function formatCPF(value) {
    value = value.replace(/\D/g, ''); // Remove non-numeric characters
    value = value.replace(/(\d{3})(\d)/, '$1.$2');
    value = value.replace(/(\d{3})(\d)/, '$1.$2');
    value = value.replace(/(\d{3})(\d{1,2})$/, '$1-$2');
    return value;
}

function validateCPF(cpf) {
    cpf = cpf.replace(/[^\d]+/g,''); // Remove non-numeric characters
    
    if (cpf.length !== 11 || /^(\d)\1{10}$/.test(cpf)) {
        return false;
    }

    let sum = 0;
    let remainder;

    for (let i = 1; i <= 9; i++) {
        sum += parseInt(cpf.substring(i - 1, i)) * (11 - i);
    }

    remainder = (sum * 10) % 11;

    if (remainder === 10 || remainder === 11) {
        remainder = 0;
    }

    if (remainder !== parseInt(cpf.substring(9, 10))) {
        return false;
    }

    sum = 0;

    for (let i = 1; i <= 10; i++) {
        sum += parseInt(cpf.substring(i - 1, i)) * (12 - i);
    }

    remainder = (sum * 10) % 11;

    if (remainder === 10 || remainder === 11) {
        remainder = 0;
    }

    if (remainder !== parseInt(cpf.substring(10, 11))) {
        return false;
    }

    return true;
}

function formatCelular(value) {
    value = value.replace(/\D/g, ''); // Remove non-numeric characters
    value = value.replace(/(\d{2})(\d)/, '($1) $2');
    value = value.replace(/(\d{5})(\d)/, '$1-$2');
    return value;
}

function convertFormDataJson(data){
    const json = {};
    data.forEach((value, key) => {
        json[key] = value;
    });
    return json;
}