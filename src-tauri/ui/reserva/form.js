const { invoke } = window.__TAURI__.tauri;
const urlParams = new URLSearchParams(window.location.search);

const form = document.getElementById('emprestimo_form');
const livroInput = document.getElementById('livro');
const pessoaInput = document.getElementById('pessoa');
const comecoInput = document.getElementById('comeco');
const fimInput = document.getElementById('fim');

const livros_datalist = document.getElementById('livros');
const pessoas_datalist = document.getElementById('pessoas');
let emprestimo = undefined;

window.addEventListener('load', async (e) => {
    const id = urlParams.get('id');
    if(id){
        try{
            //Nao pode alterar -- nao presisa dessa parte
            await preencherForm();
        }catch(e){console.log(e);}
    }
    form.addEventListener('submit', async (submit) => {
        submit.preventDefault();
        await salvarEmprestimo();
    })

    try{
        const response = await invoke('listar_livros_tauri', {tituloBusca:''});
        //console.log(response);
        adicionarLivros(response);
    }catch(e){console.log(e);}
    try{
        const response = await invoke('listar_pessoas_tauri', {cpf:''});
        //console.log(response);
        adicionarPessoas(response);
    }catch(e){console.log(e);}

    preencherDefaults();
})

async function salvarEmprestimo(){
    const data = convertFormDataJson(new FormData(document.getElementById('emprestimo_form')));
    console.log(data);
    const livro_option = searchValue(livros_datalist, data.livro);
    const livro_id = livro_option.getAttribute('item_id');
    const pessoa_option = searchValue(pessoas_datalist, data.pessoa);
    const pessoa_cpf = pessoa_option.getAttribute('cpf');

    const novo_emprestimo = {
        livroId: Number(livro_id),
        pessoaCpf: pessoa_cpf,
        comecoIsoDate: data.comeco,
        fimIsoDate: data.fim,
    }
    console.log(novo_emprestimo);
    if(emprestimo){ 
        novo_emprestimo.id = emprestimo.id 
        //Nao pode alterar emprestimos
        // try{
        //     const response = invoke('', novo_emprestimo);
        // }catch(e){ console.log(e); }
    }else{
        try{
            const response = await invoke('criar_emprestimo_tauri', novo_emprestimo);
            emprestimo = JSON.parse(response);
        }catch(e){ console.log(e); }
    }

    window.location.href = `./index.html?id=${emprestimo.id}`;
}

function searchValue(datalist, value) {
    const options = datalist.options;
    for (let i = 0; i < options.length; i++) {
        if (options[i].value === value) {
            return options[i];
        }
    }
    return null;
}

function adicionarLivros(response){
    const livros = JSON.parse(response);
    for(const livro of livros){
        livros_datalist.innerHTML += `<option value="${livro.titulo} - Volume: ${livro.volume} - Edição: ${livro.edicao}" item_id="${livro.id}"></option>`;
    }
}

function adicionarPessoas(response){
    const pessoas = JSON.parse(response);
    for(const pessoa of pessoas){
        pessoas_datalist.innerHTML += `<option value="${pessoa.nome} - ${pessoa.email}" cpf="${pessoa.cpf}"></option>`;
    }
}

function preencherDefaults(){
    livroInput.value = '';
    pessoaInput.value = '';
    comecoInput.value = getTodayString();
    fimInput.value = getTodayString(7);
}

function getTodayString(plus = 0){
    const today = addDaysToDate(new Date(), plus);
    const year = today.getFullYear();
    const month = String(today.getMonth() + 1).padStart(2, '0');
    const day = String(today.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
}

function addDaysToDate(date, days) {
    // Create a new Date object to avoid modifying the original date
    const result = new Date(date);
    // Add the specified number of days
    result.setDate(result.getDate() + days);
    return result;
}

function convertFormDataJson(data){
    const json = {};
    data.forEach((value, key) => {
        json[key] = value;
    });
    return json;
}