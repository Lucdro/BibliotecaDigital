use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::models::Idioma;
use bibliotecadigital::schema::idiomas;
use bibliotecadigital::schema::idiomas::dsl::*;

pub fn criar_idioma(conn: &mut PgConnection, nome_novo: &str) -> Result<Idioma,Error>{
    let nova_idioma = Idioma  {id: 0, nome: nome_novo.to_string()};

    diesel::insert_into(idiomas::table)
        .values(&nova_idioma)
        .returning(Idioma::as_returning())
        .get_result(conn)
}

pub fn listar_idiomas(conn: &mut PgConnection, filtro: &str) -> Result<Vec<Idioma>,Error>{
    diesel::QueryDsl::filter(idiomas, nome.ilike(format!("%{}%",filtro)))
    .load::<Idioma>(conn)
}

pub fn apagar_idioma(conn: &mut PgConnection, id_delete: i32) -> Result<usize,Error>{
    diesel::delete(idiomas.find(id_delete))
    .execute(conn)
}

pub fn mudar_nome_idioma(conn: &mut PgConnection, id_idioma: i32, novo_nome: &str) -> Result<Idioma,Error>{
    diesel::update(idiomas.find(id_idioma))
        .set(nome.eq(novo_nome))
        .returning(Idioma::as_returning())
        .get_result(conn)
}