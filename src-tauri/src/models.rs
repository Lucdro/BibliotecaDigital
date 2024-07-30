extern crate chrono;
extern crate serde;
extern crate serde_derive;

use diesel::prelude::*;
use crate::schema::*;
use serde_derive::{Serialize,Deserialize};
use chrono::{NaiveDateTime,NaiveDate};

#[derive(Queryable,Selectable,Identifiable,Insertable,Serialize,Deserialize,Debug)]
#[diesel(table_name = categorias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Categoria{
    pub id: i32,
    pub nome: String,
}
#[derive(Insertable)]
#[diesel(table_name = categorias)]
pub struct NewCategoria {
    pub nome: String,
}
#[derive(Queryable,Selectable,Identifiable,Insertable,Serialize,Deserialize,Debug)]
#[diesel(table_name = editoras)]
pub struct Editora{
    pub id: i32,
    pub nome: String,
}
#[derive(Insertable)]
#[diesel(table_name = editoras)]
pub struct NewEditora {
    pub nome: String,
}

#[derive(Queryable,Selectable,Identifiable,Insertable,Serialize,Deserialize,Debug)]
#[diesel(table_name = idiomas)]
pub struct Idioma{
    pub id: i32,
    pub nome: String,
}
#[derive(Insertable)]
#[diesel(table_name = idiomas)]
pub struct NewIdioma{
    pub nome: String,
}
#[derive(Queryable,Selectable,Insertable,Identifiable,Associations,AsChangeset,Serialize,Deserialize,Debug)]
#[diesel(belongs_to(Editora, foreign_key = editora_id))]
#[diesel(belongs_to(Idioma, foreign_key = idioma_id))]
#[diesel(table_name = livros)]
pub struct Livro{
    pub id: i32,
    pub codigo_barras: Option<String>,
    pub titulo: String,
    pub quantidade: i16,
    pub paginas: Option<i16>,
    pub publicacao: Option<i16>,
    pub editora_id: i32,
    pub edicao: String,
    pub volume: i16,
    pub idioma_id: i32,
    pub origem: Option<String>,
    pub descricao: Option<String>,
}
#[derive(Insertable)]
#[diesel(belongs_to(Editora, foreign_key = editora_id))]
#[diesel(belongs_to(Idioma, foreign_key = idioma_id))]
#[diesel(table_name = livros)]
pub struct NewLivro{
    pub codigo_barras: Option<String>,
    pub titulo: String,
    pub quantidade: i16,
    pub paginas: Option<i16>,
    pub publicacao: Option<i16>,
    pub editora_id: i32,
    pub edicao: String,
    pub volume: i16,
    pub idioma_id: i32,
    pub origem: Option<String>,
    pub descricao: Option<String>,
}
#[derive(AsChangeset)]
#[diesel(belongs_to(Editora, foreign_key = editora_id))]
#[diesel(belongs_to(Idioma, foreign_key = idioma_id))]
#[diesel(table_name = livros)]
pub struct LivroAlterar<'a>{
    pub id: i32,
    pub codigo_barras: Option<&'a str>,
    pub titulo: Option<&'a str>,
    pub quantidade: Option<i16>,
    pub paginas: Option<i16>,
    pub publicacao: Option<i16>,
    pub editora_id: Option<i32>,
    pub edicao: Option<&'a str>,
    pub volume: Option<i16>,
    pub idioma_id: Option<i32>,
    pub origem: Option<&'a str>,
    pub descricao: Option<&'a str>,
}

#[derive(Queryable,Selectable,Identifiable,Associations,Insertable,Serialize,Deserialize,Debug)]
#[diesel(belongs_to(Livro, foreign_key = livro_id))]
#[diesel(belongs_to(Categoria, foreign_key = categoria_id))]
#[diesel(table_name = categorias_livro)]
pub struct CategoriaLivro{
    pub id: i32,
    pub livro_id: i32,
    pub categoria_id: i32,
}
#[derive(Insertable)]
#[diesel(belongs_to(Livro, foreign_key = livro_id))]
#[diesel(belongs_to(Categoria, foreign_key = categoria_id))]
#[diesel(table_name = categorias_livro)]
pub struct NewCategoriaLivro{
    pub livro_id: i32,
    pub categoria_id: i32,
}

#[derive(Queryable,Selectable,Identifiable,Insertable,Serialize,Deserialize,Debug)]
#[diesel(table_name = autores)]
pub struct Autor{
    pub id: i32,
    pub nome: String,
}
#[derive(Insertable)]
#[diesel(table_name = autores)]
pub struct NewAutor{
    pub nome: String,
}

#[derive(Queryable,Identifiable,Selectable,Associations,Insertable,Serialize,Deserialize,Debug)]
#[diesel(belongs_to(Livro, foreign_key = livro_id))]
#[diesel(belongs_to(Autor, foreign_key = autor_id))]
#[diesel(table_name = autores_livro)]
pub struct AutorLivro{
    pub id: i32,
    pub livro_id: i32,
    pub autor_id: i32,
}
#[derive(Insertable)]
#[diesel(belongs_to(Livro, foreign_key = livro_id))]
#[diesel(belongs_to(Autor, foreign_key = autor_id))]
#[diesel(table_name = autores_livro)]
pub struct NewAutorLivro{
    pub livro_id: i32,
    pub autor_id: i32,
}

#[derive(Queryable,Identifiable,Selectable,Insertable,Serialize,Deserialize,Debug)]
#[diesel(primary_key(cpf))]
#[diesel(table_name = pessoas)]
pub struct Pessoa{
    pub cpf: String,
    pub nome: String,
    pub email: Option<String>,
    pub celular: Option<String>,
}
#[derive(AsChangeset)]
#[diesel(table_name = pessoas)]
pub struct PessoaAlterar<'a>{
    pub cpf: &'a str,
    pub nome: Option<&'a str>,
    pub email: Option<&'a str>,
    pub celular: Option<&'a str>,
}

#[derive(Queryable,Identifiable,Selectable,Associations,Insertable,Serialize,Deserialize,Debug)]
#[diesel(belongs_to(Livro, foreign_key = livro_id))]
#[diesel(belongs_to(Pessoa, foreign_key = pessoa_cpf))]
#[diesel(table_name = emprestimos)]
pub struct Emprestimo{
    pub id: i32,
    pub pessoa_cpf: String,
    pub livro_id: i32,
    pub comeco: NaiveDate,
    pub fim: Option<NaiveDate>,
    pub cancelado: bool,
}
#[derive(Insertable)]
#[diesel(belongs_to(Livro, foreign_key = livro_id))]
#[diesel(belongs_to(Pessoa, foreign_key = pessoa_cpf))]
#[diesel(table_name = emprestimos)]
pub struct NewEmprestimo{
    pub pessoa_cpf: String,
    pub livro_id: i32,
    pub comeco: NaiveDate,
    pub fim: Option<NaiveDate>,
    pub cancelado: bool,
}
#[derive(Queryable,Identifiable,Selectable,Insertable,Serialize,Deserialize,Debug)]
#[diesel(table_name = usuarios)]
pub struct Usuario{
    pub id: i32,
    pub nome: String,
    pub hashed_senha: String,
    pub criado_em: Option<NaiveDateTime>,   
}
#[derive(Insertable)]
#[diesel(table_name = usuarios)]
pub struct NewUsuario{
    pub nome: String,
    pub hashed_senha: String,
    pub criado_em: Option<NaiveDateTime>,   
}

#[derive(Queryable,Identifiable,Selectable,Insertable,Serialize,Deserialize,Debug)]
#[diesel(table_name = logs)]
pub struct Log{
    pub id: i32,
    pub descricao: String,
    pub criado_em: Option<NaiveDateTime>,
}
#[derive(Insertable)]
#[diesel(table_name = logs)]
pub struct NewLog{
    pub descricao: String,
    pub criado_em: Option<NaiveDateTime>,
}