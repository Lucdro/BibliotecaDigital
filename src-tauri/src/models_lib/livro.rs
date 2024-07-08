use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::schema::livros::dsl::*;
use bibliotecadigital::models::{Livro,LivroAlterar};
use bibliotecadigital::schema::livros;

pub fn criar_livro(
    conn: &mut PgConnection, 
    codigo_barras_novo: Option<&str>, 
    titulo_novo: &str, 
    quantidade_novo: i16,
    paginas_novo: Option<i16>,
    publicacao_novo: Option<i16>,
    editora_id_novo: i32,
    edicao_novo: &str,
    volume_novo: i16,
    idioma_id_novo: i32,
    origem_novo: Option<&str>,
    descricao_novo: Option<&str>,
) -> Result<Livro, Error>{
    let novo_livro = Livro  {
        id: 0,
        codigo_barras: codigo_barras_novo.map(|s| s.to_string()),
        titulo: titulo_novo.to_string(),
        quantidade: quantidade_novo,
        paginas: paginas_novo,
        publicacao: publicacao_novo,
        editora_id: editora_id_novo,
        edicao: edicao_novo.to_string(),
        volume: volume_novo,
        idioma_id: idioma_id_novo,
        origem: origem_novo.map(|s| s.to_string()),
        descricao: descricao_novo.map(|s| s.to_string()),
    };

    insert_into(livros::table)
        .values(novo_livro)
        .returning(Livro::as_returning())
        .get_result(conn)
}

pub fn listar_livros(conn: &mut PgConnection, 
    titulo_busca: Option<&str>, 
    codigo_barras_busca: Option<&str>,
    quantidade_busca: Option<i16>,
    paginas_busca: Option<i16>,
    publicacao_busca: Option<i16>,
    editora_id_busca: Option<i32>,
    edicao_busca: Option<&str>,
    volume_busca: Option<i16>,
    idioma_id_busca: Option<i32>,
    origem_busca: Option<&str>,
    descricao_busca: Option<&str>,
) -> Result<Vec<Livro>,Error>{
    let mut query = livros::table.into_boxed();

    if titulo_busca.is_some(){ query = query.filter(titulo.ilike(format!("%{}%",titulo_busca.unwrap()))); }
    if codigo_barras_busca.is_some(){ query = query.filter(codigo_barras.ilike(format!("%{}%",codigo_barras_busca.unwrap()))); }
    if quantidade_busca.is_some(){ query = query.filter(quantidade.eq(quantidade_busca.unwrap())); }
    if paginas_busca.is_some(){ query = query.filter(paginas.eq(paginas_busca.unwrap())); }
    if publicacao_busca.is_some(){ query = query.filter(publicacao.eq(publicacao_busca.unwrap())); }
    if editora_id_busca.is_some(){ query = query.filter(editora_id.eq(editora_id_busca.unwrap())); }
    if edicao_busca.is_some(){ query = query.filter(edicao.ilike(format!("%{}%",edicao_busca.unwrap()))); }
    if volume_busca.is_some(){ query = query.filter(volume.eq(volume_busca.unwrap())); }
    if idioma_id_busca.is_some(){ query = query.filter(idioma_id.eq(idioma_id_busca.unwrap())); }
    if origem_busca.is_some(){ query = query.filter(origem.ilike(format!("%{}%",origem_busca.unwrap()))); }
    if descricao_busca.is_some(){ query = query.filter(descricao.ilike(format!("%{}%",descricao_busca.unwrap()))); }

    query.load::<Livro>(conn)
}

pub fn apagar_livro(conn: &mut PgConnection, id_delete: i32) -> Result<usize, Error>{
    diesel::delete(livros.find(id_delete))
    .execute(conn)
}

pub fn alterar_livro(conn: &mut PgConnection, 
    id_alterar: i32,
    titulo_alterar: Option<&str>, 
    codigo_barras_alterar: Option<&str>,
    quantidade_alterar: Option<i16>,
    paginas_alterar: Option<i16>,
    publicacao_alterar: Option<i16>,
    editora_id_alterar: Option<i32>,
    edicao_alterar: Option<&str>,
    volume_alterar: Option<i16>,
    idioma_id_alterar: Option<i32>,
    origem_alterar: Option<&str>,
    descricao_alterar: Option<&str>,
) -> Result<Livro, Error>{
    let livro_alterar = LivroAlterar{
        id: id_alterar,
        titulo: titulo_alterar,
        codigo_barras: codigo_barras_alterar,
        quantidade: quantidade_alterar,
        paginas: paginas_alterar,
        publicacao: publicacao_alterar,
        editora_id: editora_id_alterar,
        edicao: edicao_alterar,
        volume: volume_alterar,
        idioma_id: idioma_id_alterar,
        origem: origem_alterar,
        descricao: descricao_alterar,
    };
    diesel::update(livros::table.find(id_alterar))
    .set(livro_alterar)
    .returning(Livro::as_returning())
    .get_result(conn)
}