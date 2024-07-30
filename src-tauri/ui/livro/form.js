// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri;

const form = document.getElementById('livro_form');
const urlParams = new URLSearchParams(window.location.search);
const selecao_categorias = document.getElementById('selecao_categorias');
const selecao_autores = document.getElementById('selecao_autores');
const idiomas_datalist = document.getElementById('idiomas');
const editoras_datalist = document.getElementById('editoras');
const categorias_datalist = document.getElementById('categorias');
const categorias_selecionadas = document.getElementById('categorias_selecionadas');
const autores_datalist = document.getElementById('autores');
const autores_selecionadas = document.getElementById('autores_selecionadas');
let livro = undefined;
let editoras = undefined;
let idiomas = undefined;
let categorias = undefined;
let autores = undefined;

function validateNumberInput(event) {
    const input = event.target;
    input.value = input.value.replace(/[^0-9]/g, '');
}

function convertFormDataJson(data){
    const json = {};
    data.forEach((value, key) => {
        json[key] = value;
    });
    return json;
}

function capitalizeFirstLetter(word) {
    return word.charAt(0).toUpperCase() + word.slice(1);
}

function capitalizeSentence(sentence) {
    return sentence.split(' ')
                   .map(word => capitalizeFirstLetter(word))
                   .join(' ');
}

function ensureRange(event){
    const input = event.target;
    const min = parseInt(input.min, 10);
    const max = parseInt(input.max, 10);
    const value = parseInt(input.value, 10);

    // Clamp the value between min and max
    input.value = Math.max(min, Math.min(max, value));
}

window.addEventListener('load', async (e) => {
    const id = urlParams.get('id');
    if(id){
        try{
            const response = await invoke('buscar_livro_id_tauri', {id: parseInt(urlParams.get('id'), 10)});
            livro = JSON.parse(response);
            await preencherForm();
        }catch(e){console.log(e);}
    }
    await adicionarEventosCategoria();
    await adicionarEventosAutores();
    form.addEventListener('submit', async (submit) => {
        submit.preventDefault();
        await salvarLivro();
    })
    try{
        const response = await invoke('listar_idiomas_tauri', {nome:''});
        adicionarIdiomas(response);
    }catch(e){console.log(e);}
    try{
        const response = await invoke('listar_editoras_tauri', {nome:''});
        adicionarEditoras(response);
    }catch(e){console.log(e);}
    try{
        const response = await invoke('listar_categorias_tauri', {nome:''});
        adicionarCategorias(response);
    }catch(e){console.log(e);}
    try{
        const response = await invoke('listar_autores_tauri', {nome:''});
        adicionarAutores(response)
    }catch(e){console.log(e);}
})
async function preencherForm(){
    //Preenche inputs
    let editora = undefined;
    try{
        let response = await invoke('buscar_editora_id_tauri',{ id: Number(livro.editora_id)});
        editora = JSON.parse(response);
    }catch(e){console.log(e);}
    let idioma = undefined;
    try{
        let response = await invoke('buscar_idioma_id_tauri',{ id: Number(livro.idioma_id)});
        idioma = JSON.parse(response);
    }catch(e){console.log(e);}
    data = {
        codigo_barras: livro.codigo_barras,
        titulo: livro.titulo,
        quantidade: livro.quantidade,
        paginas: livro.paginas,
        publicacao: livro.publicacao,
        editora: editora?.nome,
        edicao: livro.edicao,
        volume: livro.volume,
        idioma: idioma?.nome,
        origem: livro.origem,
        descricao: livro.descricao,
    };
    Object.keys(data).forEach(key => {
        const field = form.querySelector(`input[name=${key}]`);
        if (field) field.value = data[key];
    });
    //Preenche divs
    //Categorias
    try{
        const response = await invoke('listar_categorias_livro_tauri', {idLivro: livro.id} );
        const categorias_atuais = JSON.parse(response);
        for(const categoria of categorias_atuais){
            addSelectionContentCategoria(categoria.nome, categoria.id);   
        }
    }catch(e){console.log(e);}
    //Autores
    try{
        const response = await invoke('listar_autores_livro_tauri', {idLivro: livro.id} );
        const autores_atuais = JSON.parse(response);
        for(const autor of autores_atuais){
            addSelectionContentAutores(autor.nome, autor.id);   
        }
    }catch(e){console.log(e);}
}
async function adicionarEventosCategoria(){
    document.getElementById('add_selection_categoria').addEventListener('click', (e) => {
        e.preventDefault();
        selecao_categorias.classList.toggle('hidden');
    })

    selecao_categorias.addEventListener('input', (e) =>{
        if(e.data == undefined && e.inputType != 'deleteContentBackward'){
            const categoria = categorias_datalist.querySelector(`option[value="${e.target.value}"]`);
            addSelectionContentCategoria(categoria.value, categoria.getAttribute('item_id'));
            e.target.value = '';
            e.target.classList.toggle('hidden');
            categorias_datalist.classList.toggle('hidden');
        }
    })
    selecao_categorias.addEventListener('keydown', (e) => {
        if(e.key == 'Escape'){
            selecao_categorias.value = '';
            selecao_categorias.style.display = 'none' 
        }else if(e.key == 'Enter'){
            const value = e.target.value;
            selecao_categorias.value = '';
            selecao_categorias.classList.toggle('hidden');
            categorias_datalist.classList.toggle('hidden');
            if(value == ''){return;}
            addSelectionContentCategoria(value);
            e.preventDefault();
        }
    })
}

function addSelectionContentCategoria(value, item_id){
    if(value == ''){
        selecao_categorias.value = '';
        selecao_categorias.classList.toggle('hidden');
        return;
    };
    const find = categorias_selecionadas.querySelector(`div[value="${value.toLowerCase()}"]`)
    if(find && item_id) {
        if(find.getAttribute('item_id')){ return; }
        else{ find.remove(); }
    }
    if(item_id){
        categorias_selecionadas.innerHTML += `<div value="${value.toLowerCase()}" item_id="${item_id}" onclick="deletarElemento(event)">${capitalizeFirstLetter(value)}</div>`;
    }
    else{
        categorias_selecionadas.innerHTML += `<div value="${value.toLowerCase()}" onclick="deletarElemento(event)">${capitalizeFirstLetter(value)}</div>`;
    }
}
function deletarElemento(e){
    e.target.remove();
}

async function adicionarEventosAutores(){
    document.getElementById('add_selection_autor').addEventListener('click', (e) => {
        e.preventDefault();
        document.getElementById('selecao_autores').classList.toggle('hidden');
    })
    selecao_autores.addEventListener('input', (e) =>{
        if(e.data == undefined && e.inputType != 'deleteContentBackward'){
            const autor = autores_datalist.querySelector(`option[value="${e.target.value}"]`);
            addSelectionContentAutores(autor.value, autor.getAttribute('item_id'));
            e.target.value = '';
            e.target.classList.toggle('hidden');
            autores_datalist.classList.toggle('hidden');
        }
    })
    selecao_autores.addEventListener('keydown', (e) => {
        if(e.key == 'Escape'){
            selecao_autores.value = '';
            selecao_autores.style.display = 'none' 
        }else if(e.key == 'Enter'){
            const value = e.target.value;
            selecao_autores.value = '';
            selecao_autores.classList.toggle('hidden');
            autores_datalist.classList.toggle('hidden');
            if(value == ''){return;}
            addSelectionContentAutores(value);
            e.preventDefault();
        }
    })

    
}

function addSelectionContentAutores(value, item_id){
    if(value == ''){
        selecao_autores.value = '';
        selecao_autores.classList.toggle('hidden');
        return
    };
    const find = autores_selecionadas.querySelector(`div[value="${value.toLowerCase()}"]`)
    if(find && item_id) {
        if(find.getAttribute('item_id')){ return; }
        else{ find.remove(); }
    }
    if(item_id){
        autores_selecionadas.innerHTML += `<div value="${value.toLowerCase()}" item_id="${item_id}" onclick="deletarElemento(event)">${capitalizeSentence(value)}</div>`;
    }
    else{
        autores_selecionadas.innerHTML += `<div value="${value.toLowerCase()}" onclick="deletarElemento(event)">${capitalizeSentence(value)}</div>`;
    }
}

function adicionarEditoras(response){
    editoras = JSON.parse(response);
    for(const editora of editoras)
    {
        editoras_datalist.innerHTML += `<option value="${editora.nome}" item_id="${editora.id}"></option>`;
    }
}

function adicionarIdiomas(response){
    idiomas = JSON.parse(response);
    for(const idioma of idiomas){
        idiomas_datalist.innerHTML += `<option value="${idioma.nome}" item_id="${idioma.id}"></option>`;
    }

}
function adicionarCategorias(response){
    categorias = JSON.parse(response);
    for(const categoria of categorias){
        categorias_datalist.innerHTML += `<option value="${categoria.nome}" item_id="${categoria.id}"></option>`;
    }
}
function adicionarAutores(response){
    autores = JSON.parse(response);
    for(const autor of autores){
        autores_datalist.innerHTML += `<option value="${autor.nome}" item_id="${autor.id}"></option>`;
    }
}

async function salvarLivro(){
    const data = convertFormDataJson(new FormData(document.getElementById('livro_form')));
    console.log(data);
    let novo_livro = {};
    
    novo_livro.editoraIdNovo = await verificarCriarEditora(data);
    novo_livro.idiomaIdNovo = await verificarCriarIdioma(data);
    const ids_categoria = await verificarCriarCategoria();
    const ids_autor = await verificarCriarAutor();    

    //verificar e criar livro
    novo_livro.codigoBarrasNovo = data.codigo_barras == '' ? undefined : data.codigo_barras; 
    novo_livro.tituloNovo = data.titulo;
    novo_livro.quantidadeNovo = Number(data.quantidade);
    novo_livro.paginasNovo = data.paginas == '' ? undefined : Number(data.paginas);
    novo_livro.publicacaoNovo = data.publicacao == '' ? undefined : Number(data.publicacao);
    novo_livro.edicaoNovo = data.edicao;
    novo_livro.volumeNovo = Number(data.volume);
    novo_livro.origemNovo = data.origem == '' ? undefined : data.origem;
    novo_livro.descricaoNovo = data.descricao == '' ? undefined : data.descricao;
    console.log(novo_livro);
    if(livro){
        novo_livro.idAlterar = Number(livro.id);
        try{
            const response = await invoke('alterar_livro_tauri',novo_livro);
            livro = JSON.parse(response);
        }catch(e){console.log(e);}
    }else{
        try{
            const response = await invoke('salvar_livro_tauri',novo_livro);
            livro = JSON.parse(response);
        }catch(e){console.log(e);}
    }

    //verificar e criar categoriaslivro 
    let categorias_antigas = [];
    try{
        const response = await invoke('listar_categorias_livro_tauri', {idLivro: Number(livro.id)});
        categorias_antigas = JSON.parse(response);
    }catch(e){console.log(e);}
    let new_categorias = [];
    for(const id of ids_categoria){
        if(!categorias_antigas.find(categoria => categoria.id == id)){
            try{
                const response = await invoke('adicionar_categoria_livro_tauri', {idLivro: Number(livro.id), idCategoria: Number(id)});
                const categoria = JSON.parse(response);
                new_categorias.push(Number(categoria.categoria_id));
            }catch(e){console.log(e);}
        }else{ new_categorias.push(Number(id));}
    }
    //deletar categorias sobrando
    let categorias_remover = categorias_antigas.filter( categoria_antiga => !new_categorias.find(id => id === categoria_antiga.id));
    //console.log(categorias_antigas,new_categorias,categorias_remover);
    for(const categoria of categorias_remover){
        try{
            await invoke('remover_categoria_livro_tauri', {idCategoria: Number(categoria.id), idLivro: Number(livro.id) });
        }catch(e){console.log(e);}
    }

    //verificar e criar autoreslivro
    let autores_antigos = [];
    try{
        const response = await invoke('listar_autores_livro_tauri', {idLivro: Number(livro.id)});
        autores_antigos = JSON.parse(response);
    }catch(e){console.log(e);}
    let new_autores = [];
    for(const id of ids_autor){
        if(!autores_antigos.find(autor => autor.id == id)){
            try{
                const response = await invoke('adicionar_autor_livro_tauri', {idLivro: Number(livro.id), idAutor: Number(id)});
                const autor = JSON.parse(response);
                new_autores.push(autor.autor_id);
            }catch(e){console.log(e);}
        }else{ new_autores.push(id); }
    }
    //deletar autores sobrando
    let autores_remover = autores_antigos.filter( autor_antigo => !new_autores.find(id => id === autor_antigo.id));
    //console.log(autores_antigos,new_autores,autores_remover)
    for(const autor of autores_remover){
        try{
            const response = await invoke('remover_autor_livro_tauri', {idAutor: Number(autor.id), idLivro: Number(livro.id) });
        }catch(e){console.log(e);}
    }
    window.location.href = `./index.html?titulo=${livro.titulo}`;
}

async function verificarCriarEditora(data){
    //verificar e criar editora
    editora_salva = editoras.find(editora => editora.nome == data.editora);
    if(!editora_salva){
        try{
            const response = await invoke('salvar_editora_tauri', {nome: data.editora});
            let editora = JSON.parse(response);
            return Number(editora.id);
        }catch(e){console.log(e);}
    }else{ return Number(editora_salva.id); }
}

async function verificarCriarIdioma(data){
    //verificar e criar idioma
    idioma_salvo = idiomas.find(idioma => idioma.nome == data.idioma);
    if(!idioma_salvo){
        try{
            const response = await invoke('salvar_idioma_tauri', {nome: data.idioma});
            let idioma = JSON.parse(response);
            return Number(idioma.id);
        }catch(e){console.log(e);}
    }else{ return Number(idioma_salvo.id); }
}

async function verificarCriarCategoria(){
    let ids_categoria = [];
    for(const div of categorias_selecionadas.querySelectorAll('div')){
        let id = div.getAttribute('item_id');
        if(id == undefined){
            try{
                const response = await invoke('salvar_categoria_tauri', {nome: div.getAttribute('value')});
                const categoria  = JSON.parse(response);
                id = categoria.id;
            }catch(e){ console.log(e); }
        }
        ids_categoria.push(Number(id));
    }
    return ids_categoria;
}

async function verificarCriarAutor(){
    let ids_autor = [];
    for(const div of autores_selecionadas.querySelectorAll('div')){
        let id = div.getAttribute('item_id');
        if(id == undefined){
            try{
                const response = await invoke('criar_autor_tauri', {nome: div.getAttribute('value')});
                const autor  = JSON.parse(response);
                id = autor.id;
            }catch(e){console.log(e);}
        }
        ids_autor.push(Number(id));
    }
    return ids_autor;
}