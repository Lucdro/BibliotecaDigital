// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri;

const lista_livros = document.getElementById('lista_livros');

window.addEventListener('load', async function() {
    document.getElementById('filters_form').addEventListener("submit", async function(event) {
        // Prevent the default form submission
        event.preventDefault();
    
        // Get form data
        const formData = new FormData(document.getElementById('filters_form'));
    
        // Convert form data to an object
        const data = convertFormDataJson(formData);

        await buscarLivros(data.titulo);
    })
    document.getElementById('create_new').addEventListener("click", function(event){
        window.location.href = './form.html';
    })

    await buscarLivros('');
});

function convertFormDataJson(data){
    const json = {};
    data.forEach((value, key) => {
        json[key] = value;
    });
    return json;
}

async function buscarLivros(filtro){
    try{
        const response = await invoke('listar_livros_tauri', {tituloBusca: filtro});
        //console.log(filtro);
        const livros = JSON.parse(response);
        //console.log(livros)
        lista_livros.innerHTML = '';
        for(const livro of livros){
            criarElementoLivro(livro);
        }
    }catch(e){console.log(e);}
}

async function criarElementoLivro(livro){
    let autores = [];
    try{
        const response = await invoke('listar_autores_livro_tauri', {idLivro: Number(livro.id)});
        autores = JSON.parse(response);
    }catch(e){console.log(e);}

    let autoresString = '<div class="container_autores">';
    for(const autor of autores){
        autoresString += `<span class="autor">${autor.nome}</span>`;
    }
    autoresString += '</div>';

    lista_livros.innerHTML += `
        <div class="item_livro" id_livro="${livro.id}">
            <div>
                <span class="titulo">${livro.titulo}</span>
                ${autoresString}
            </div>
            <div>
                <button class="alterar" onclick="alterarLivro(${livro.id})">Alterar</button>
                <button class="apagar" onclick="apagarLivro(${livro.id});">Apagar</button> 
            </div>
        </div>
    `
}

function alterarLivro(id){
    window.location.href = './form.html?id='+id;
}

async function apagarLivro(id){
    if(! await confirm("Deseja apagar o livro?")){ return; }
    
    //Apagar autores do livro
    try{
        const response = await invoke('listar_autores_livro_tauri', {idLivro: Number(id)});
        const autores = JSON.parse(response);

        for(const autor of autores){
            try{
                await invoke('remover_autor_livro_tauri', {idAutor: autor.id, idLivro: id});
            }catch(e){ console.log(e); }
        }
    }catch(e){ console.log(e); }
    
    //Apagar categorias do livro
    try{
        const response = await invoke('listar_categorias_livro_tauri', {idLivro: Number(id)});
        const categorias = JSON.parse(response);
        
        for(const categoria of categorias){
            try{
                await invoke('remover_categoria_livro_tauri', {idCategoria: categoria.id, idLivro: id});
            }catch(e){ console.log(e); }
        }
    }catch(e){ console.log(e); }

    //Apagar Livro
    try{
        await invoke('apagar_livro_tauri' , {idApagar: id});
    }catch(e){ console.log(e); }
    location.reload();
}