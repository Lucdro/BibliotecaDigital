use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::models::{Pessoa,PessoaAlterar};
use bibliotecadigital::schema::pessoas;
use bibliotecadigital::schema::pessoas::dsl::*;

pub fn criar_pessoa(conn: &mut PgConnection,
    cpf_novo: &str, 
    nome_novo: &str,
    email_novo: Option<&str>,
    celular_novo: Option<&str>,
) -> Result<Pessoa,Error>{
    let nova_pessoa = Pessoa {
        cpf: cpf_novo.to_string(), 
        nome: nome_novo.to_string(),
        email: email_novo.map(|s| s.to_string()),
        celular: celular_novo.map(|s| s.to_string()),
    };

    diesel::insert_into(pessoas::table)
        .values(&nova_pessoa)
        .returning(Pessoa::as_returning())
        .get_result(conn)
}

pub fn listar_pessoas(conn: &mut PgConnection, 
    cpf_busca: Option<&str>,
    nome_busca: Option<&str>,
    email_busca: Option<&str>,
    celular_busca: Option<&str>,
) -> Result<Vec<Pessoa>,Error>{
    let mut query = pessoas::table.into_boxed();
    
    if cpf_busca.is_some() { query = query.filter(cpf.ilike(format!("%{}%",cpf_busca.unwrap()))); }
    if nome_busca.is_some() { query = query.filter(nome.ilike(format!("%{}%",nome_busca.unwrap()))); }
    if email_busca.is_some() { query = query.filter(email.ilike(format!("%{}%",email_busca.unwrap()))); }
    if celular_busca.is_some() { query = query.filter(celular.ilike(format!("%{}%",celular_busca.unwrap()))); }

    query.load::<Pessoa>(conn)
}

pub fn apagar_pessoa(conn: &mut PgConnection, cpf_delete: &str) -> Result<usize,Error>{
    diesel::delete(pessoas.filter(cpf.eq(cpf_delete)))
    .execute(conn)
}

pub fn alterar_pessoa(conn: &mut PgConnection,
    cpf_alterar: &str,
    novo_nome: Option<&str>,
    novo_email: Option<&str>,
    novo_celular: Option<&str>,
) -> Result<Pessoa,Error>{
    let pessoa_alterar = PessoaAlterar {
        cpf: cpf_alterar,
        nome: novo_nome,
        email: novo_email,
        celular: novo_celular,
    };
    diesel::update(pessoas::table.find(cpf_alterar))
    .set(pessoa_alterar)
    .returning(Pessoa::as_returning())
    .get_result(conn)
}