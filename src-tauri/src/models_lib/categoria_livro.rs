use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::models::{CategoriaLivro, NewCategoriaLivro, Livro, Categoria};
use bibliotecadigital::schema::{categorias_livro, livros, categorias};
use bibliotecadigital::schema::categorias_livro::dsl::*;
use bibliotecadigital::schema::livros::dsl::*;
use bibliotecadigital::schema::categorias::dsl::*;

pub fn adicionar_categoria_livro(conn: &mut PgConnection, novo_categoria_id: i32, novo_livro_id: i32) -> Result<CategoriaLivro,Error>{
    let nova_categoria_livro = NewCategoriaLivro  {
        categoria_id: novo_categoria_id,
        livro_id: novo_livro_id,
    };

    diesel::insert_into(categorias_livro::table)
        .values(&nova_categoria_livro)
        .returning(CategoriaLivro::as_returning())
        .get_result(conn)
}

pub fn remover_categoria_livro(conn: &mut PgConnection, id_categoria: i32, id_livro: i32) -> Result<usize,Error>{
    diesel::delete(categorias_livro.filter(categoria_id.eq(id_categoria).and(livro_id.eq(id_livro))))
    .execute(conn)
}

pub fn listar_livros_categoria(conn: &mut PgConnection, id_categoria: i32) -> Result<Vec<Livro>,Error>{
    let livros_categoria: Vec<CategoriaLivro> = diesel::QueryDsl::filter(categorias_livro, categoria_id.eq(id_categoria)).load::<CategoriaLivro>(conn)?;
    let livro_ids: Vec<i32> = livros_categoria.iter().map(|lc| lc.livro_id).collect();
    livros.filter(livros::id.eq_any(livro_ids)).load::<Livro>(conn)
}

pub fn listar_categorias_livro(conn: &mut PgConnection, id_livro: i32) -> Result<Vec<Categoria>,Error>{
    let categorias_livro_vec: Vec<CategoriaLivro> = diesel::QueryDsl::filter(categorias_livro, livro_id.eq(id_livro)).load::<CategoriaLivro>(conn)?;
    let autores_ids: Vec<i32> = categorias_livro_vec.iter().map(|ac| ac.categoria_id).collect();
    categorias.filter(categorias::id.eq_any(autores_ids)).load::<Categoria>(conn)
}