use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::models::{AutorLivro, NewAutorLivro , Livro, Autor};
use bibliotecadigital::schema::{autores_livro, livros, autores};
use bibliotecadigital::schema::autores_livro::dsl::*;
use bibliotecadigital::schema::livros::dsl::*;
use bibliotecadigital::schema::autores::dsl::*;

pub fn adicionar_autor_livro(conn: &mut PgConnection, novo_autor_id: i32, novo_livro_id: i32) -> Result<AutorLivro,Error>{
    let novo_autor_livro = NewAutorLivro  {
        autor_id: novo_autor_id,
        livro_id: novo_livro_id,
    };

    diesel::insert_into(autores_livro::table)
        .values(&novo_autor_livro)
        .returning(AutorLivro::as_returning())
        .get_result(conn)
}

pub fn remover_autor_livro(conn: &mut PgConnection, id_autor: i32, id_livro: i32) -> Result<usize,Error>{
    diesel::delete(autores_livro.filter(autor_id.eq(id_autor).and(livro_id.eq(id_livro))))
    .execute(conn)
}

pub fn listar_livros_autor(conn: &mut PgConnection, id_autor: i32) -> Result<Vec<Livro>,Error>{
    let livros_autor: Vec<AutorLivro> = diesel::QueryDsl::filter(autores_livro, autor_id.eq(id_autor)).load::<AutorLivro>(conn)?;
    let livro_ids: Vec<i32> = livros_autor.iter().filter_map(|al| al.livro_id).collect();
    livros.filter(livros::id.eq_any(livro_ids)).load::<Livro>(conn)
}

pub fn listar_autores_livro(conn: &mut PgConnection, id_livro: i32) -> Result<Vec<Autor>,Error>{
    let autores_livro_vec: Vec<AutorLivro> = diesel::QueryDsl::filter(autores_livro, livro_id.eq(id_livro)).load::<AutorLivro>(conn)?;
    let autores_ids: Vec<i32> = autores_livro_vec.iter().filter_map(|al| al.autor_id).collect();
    autores.filter(autores::id.eq_any(autores_ids)).load::<Autor>(conn)
}