use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::models::{Categoria, NewCategoria};
use bibliotecadigital::schema::categorias;
use bibliotecadigital::schema::categorias::dsl::*;

pub fn criar_categoria(conn: &mut PgConnection, nome_novo: &str) -> Result<Categoria,Error>{
    let nova_categoria = NewCategoria  {nome: nome_novo.to_string()};

    diesel::insert_into(categorias::table)
        .values(&nova_categoria)
        .returning(Categoria::as_returning())
        .get_result(conn)
}

pub fn listar_categorias(conn: &mut PgConnection, filtro: &str) -> Result<Vec<Categoria>,Error>{
    diesel::QueryDsl::filter(categorias, nome.ilike(format!("%{}%",filtro)))
    .load::<Categoria>(conn)
}

pub fn apagar_categoria(conn: &mut PgConnection, id_delete: i32) -> Result<usize,Error>{
    diesel::delete(categorias.find(id_delete))
    .execute(conn)
}

pub fn mudar_nome_categoria(conn: &mut PgConnection, id_categoria: i32, novo_nome: &str) -> Result<Categoria,Error>{
    diesel::update(categorias.find(id_categoria))
        .set(nome.eq(novo_nome))
        .returning(Categoria::as_returning())
        .get_result(conn)
}