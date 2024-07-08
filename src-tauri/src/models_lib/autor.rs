use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::models::Autor;
use bibliotecadigital::schema::autores;
use bibliotecadigital::schema::autores::dsl::*;

pub fn criar_autor(conn: &mut PgConnection, nome_novo: &str) -> Result<Autor,Error>{
    let novo_autor = Autor  {id: 0, nome: nome_novo.to_string()};

    diesel::insert_into(autores::table)
        .values(&novo_autor)
        .returning(Autor::as_returning())
        .get_result(conn)
}

pub fn listar_autores(conn: &mut PgConnection, filtro: &str) -> Result<Vec<Autor>,Error>{
    diesel::QueryDsl::filter(autores, nome.ilike(format!("%{}%",filtro)))
    .load::<Autor>(conn)
}

pub fn apagar_autor(conn: &mut PgConnection, id_delete: i32) -> Result<usize,Error>{
    diesel::delete(autores.find(id_delete))
    .execute(conn)
}

pub fn mudar_nome_autor(conn: &mut PgConnection, id_autor: i32, novo_nome: &str) -> Result<Autor,Error>{
    diesel::update(autores.find(id_autor))
        .set(nome.eq(novo_nome))
        .returning(Autor::as_returning())
        .get_result(conn)
}