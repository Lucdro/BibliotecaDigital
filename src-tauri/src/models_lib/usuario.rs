use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::models::{Usuario,NewUsuario};
use bibliotecadigital::schema::usuarios;
use bibliotecadigital::schema::usuarios::dsl::*;

pub fn criar_usuario(conn: &mut PgConnection,
    nome_novo: &str,
    hashed_senha_novo: String,
) -> Result<Usuario,Error> {
    let novo_usuario = NewUsuario {
        nome: nome_novo.to_string(),
        hashed_senha: hashed_senha_novo.to_string(),
        criado_em: None,
    };

    diesel::insert_into(usuarios::table)
        .values(&novo_usuario)
        .returning(Usuario::as_returning())
        .get_result(conn)
}

pub fn apagar_usuario(conn: &mut PgConnection, id_deletar: i32) -> Result<usize,Error>{
    diesel::delete(usuarios.find(id_deletar))
    .execute(conn)
}

pub fn listar_usuarios(conn: &mut PgConnection, nome_filtro: &str) -> Result<Vec<Usuario>,Error>{
    usuarios.filter(nome.ilike(format!("%{}%",nome_filtro)))
    .get_results(conn)
}

pub fn buscar_usuario_exato(conn: &mut PgConnection, nome_buscar: &str) -> Result<Usuario,Error>{
    usuarios.filter(nome.eq(nome_buscar))
    .get_result(conn)
}
