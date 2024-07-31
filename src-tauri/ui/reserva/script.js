// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri;

const lista_emprestimos = document.getElementById('lista_emprestimos');

window.addEventListener('load', async function() {
    document.getElementById('filters_form').addEventListener("submit", async function(event) {
        // Prevent the default form submission
        event.preventDefault();
    
        // Get form data
        const formData = new FormData(document.getElementById('filters_form'));
    
        // Convert form data to an object
        const data = convertFormDataJson(formData);

        await buscarEmprestimos(data.livro,data.pessoa,data.cancelado);
    })
    document.getElementById('create_new').addEventListener("click", function(event){
        window.location.href = './form.html';
    })

    await buscarEmprestimos('','', undefined);
});

function convertFormDataJson(data){
    const json = {};
    data.forEach((value, key) => {
        json[key] = value;
    });
    return json;
}

async function buscarEmprestimos(livro, pessoa, cancelado){
    try{
        const response = await invoke('listar_emprestimos_tauri', {canceladoBusca: cancelado === "on"});
        const emprestimos = JSON.parse(response);
        lista_emprestimos.innerHTML = '';
        for(const emprestimo of emprestimos){
            criarElementoEmprestimo(emprestimo);
        }
    }catch(e){console.log(e);}
}

async function criarElementoEmprestimo(emprestimo){
    let livro = undefined;
    let pessoa = undefined;
    console.log(emprestimo)
    try{
        const response = await invoke('buscar_livro_id_tauri',{ id: emprestimo.livro_id })
        livro = JSON.parse(response);
    }catch(e) { console.log(e); }

    try{
        const response = await invoke('listar_pessoas_tauri',{ cpf: emprestimo.pessoa_cpf })
        const pessoas = JSON.parse(response);
        if(pessoas.length > 0) pessoa = pessoas[0];
    }catch(e) { console.log(e); }

    lista_emprestimos.innerHTML += `
        <div class="item_emprestimo">
            <div>
                <span class="livro">${livro.titulo || "Livro inválido"}</span>
                <span class="pessoa">${pessoa.nome || "Pessoa inválida"}</span>
            </div>
            <div>
                <span class="date_item">Comeco: ${formatarDate(emprestimo.comeco)}</span>
                <span class="date_item">Fim: ${formatarDate(emprestimo.fim)}</span>
            </div>
            <div>
                <span class="cancelado_item">${emprestimo.cancelado ? "Cancelado" : "Ativo"}</span>
            </div>
            <div>
                <button class="apagar" onclick="cancelarEmprestimo('${emprestimo.id}');">Cancelar</button> 
            </div>
        </div>
    `
}

async function cancelarEmprestimo(id){
    if(! await confirm("Deseja cancelar o empréstimo?")){ return; }

    try{
        await invoke('cancelar_emprestimo_tauri' , {idCancelar: Number(id)});
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
function formatarDate(date_formatar){
    const date = new Date(date_formatar);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${day}/${month}/${year}`;
}