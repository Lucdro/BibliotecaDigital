use chrono::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::schema::emprestimos::dsl::*;
use bibliotecadigital::models::{Emprestimo, NewEmprestimo};
use bibliotecadigital::schema::emprestimos;

pub fn criar_emprestimo(
    conn: &mut PgConnection, 
    livro_id_novo: i32,
    pessoa_cpf_novo: &str,
    comeco_novo: NaiveDate,
    fim_novo: Option<NaiveDate>,
) -> Result<Emprestimo, Error>{
    let novo_emprestimo = NewEmprestimo  {
        livro_id: livro_id_novo,
        pessoa_cpf: pessoa_cpf_novo.to_string(),
        comeco: comeco_novo,
        fim: fim_novo,
        cancelado: false,
    };

    insert_into(emprestimos::table)
        .values(novo_emprestimo)
        .returning(Emprestimo::as_returning())
        .get_result(conn)
}

pub fn listar_emprestimos(conn: &mut PgConnection, 
    id_busca: Option<i32>,
    livro_id_busca: Option<i32>,
    pessoa_cpf_busca: Option<&str>,
    comeco_busca: Option<NaiveDate>,
    fim_busca: Option<NaiveDate>,
    cancelado_busca: Option<bool>,
) -> Result<Vec<Emprestimo>,Error>{
    let mut query = emprestimos::table.into_boxed();

    if id_busca.is_some() { query = query.filter(id.eq(id_busca.unwrap())); }
    if livro_id_busca.is_some() { query = query.filter(livro_id.eq(livro_id_busca.unwrap())); }
    if pessoa_cpf_busca.is_some() { query = query.filter(pessoa_cpf.ilike(format!("%{}%",pessoa_cpf_busca.unwrap()))); }
    if comeco_busca.is_some() { query = query.filter(comeco.ge(comeco_busca.unwrap())); }
    if fim_busca.is_some() { query = query.filter(fim.le(fim_busca.unwrap())); }
    if cancelado_busca.is_some() { query = query.filter(cancelado.eq(cancelado_busca.unwrap())); }

    query.load::<Emprestimo>(conn)
}

pub fn cancelar_emprestimo(conn: &mut PgConnection, id_cancelar: i32) -> Result<Emprestimo,Error>{
    diesel::update(emprestimos.find(id_cancelar))
        .set(cancelado.eq(true))
        .returning(Emprestimo::as_returning())
        .get_result(conn)
}