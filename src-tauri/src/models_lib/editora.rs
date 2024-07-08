use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::models::Editora;
use bibliotecadigital::schema::editoras;
use bibliotecadigital::schema::editoras::dsl::*;

pub fn criar_editora(conn: &mut PgConnection, nome_novo: &str) -> Result<Editora,Error>{
    let nova_editora = Editora  {id: 0, nome: nome_novo.to_string()};

    diesel::insert_into(editoras::table)
        .values(&nova_editora)
        .returning(Editora::as_returning())
        .get_result(conn)
}

pub fn listar_editoras(conn: &mut PgConnection, filtro: &str) -> Result<Vec<Editora>,Error>{
    diesel::QueryDsl::filter(editoras, nome.ilike(format!("%{}%",filtro)))
    .load::<Editora>(conn)
}

pub fn apagar_editora(conn: &mut PgConnection, id_delete: i32) -> Result<usize,Error>{
    diesel::delete(editoras.find(id_delete))
    .execute(conn)
}

pub fn mudar_nome_editora(conn: &mut PgConnection, id_editora: i32, novo_nome: &str) -> Result<Editora,Error>{
    diesel::update(editoras.find(id_editora))
        .set(nome.eq(novo_nome))
        .returning(Editora::as_returning())
        .get_result(conn)
}
