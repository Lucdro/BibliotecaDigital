use crate::models_lib::autor_livro::*;
use crate::models_lib::categoria_livro::*;
use crate::models_lib::categoria::*;
use crate::models_lib::editora::*;
use crate::models_lib::idioma::*;
use crate::models_lib::livro::*;
use crate::models_lib::autor::*;
use crate::models_lib::pessoa::*;
use crate::models_lib::emprestimo::*;
use crate::models_lib::usuario::*;
use crate::models_lib::logs::*;
use bibliotecadigital::*;
use chrono::NaiveDate;
use diesel::PgConnection;
use tauri::command;
use dotenvy::dotenv;
use std::env;
use core::result::Result;
use sha2::{Sha256, Digest};

fn tauri_create_log(conn: &mut PgConnection, descricao: String) -> Result<(),String> {
    dotenv().expect(".env File ERROR: create the file at src-tauri\\src and add the postgres url\nEx: DATABASE_URL=postgres://postgres:2105@localhost/biblioteca_digital");
    let user = env::var("USER").expect("Nenhum usuário encontrado em .env");
    _ = criar_log(conn, format!("{}, usuário: {}", descricao, user)).map_err(|e| format!("Não foi possível criar log: {}", e));
    Ok(())
}

//Categoria
#[command]
pub fn salvar_categoria_tauri(nome: &str) -> Result<String, String> {
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match criar_categoria(&mut conn, nome){
        Ok(categoria) => {
            let json = serde_json::to_string(&categoria);
            match json{
                Ok(j) => {
                    tauri_create_log(&mut conn, format!("Criar categoria {}",nome))?;
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível criar categoria: {}",err))}
    }
}
#[command]
pub fn listar_categorias_tauri(nome: &str) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_categorias(&mut conn, nome){
        Ok(categorias) => {
            let json = serde_json::to_string(&categorias);
            match json{
                Ok(j) => Ok(j),
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível listar categorias: {}",err))}
    }
}
#[command]
pub fn apagar_categoria_tauri(id: i32) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;
   
    match apagar_categoria(&mut conn, id){
        Ok(categoria) => {
            let json = serde_json::to_string(&categoria);
            match json{
                Ok(j) => {
                    tauri_create_log(&mut conn, format!("Apagar categoria {}",id))?;
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível apagar categoria: {}",err))}
    }
}
#[command]
pub fn mudar_nome_categoria_tauri(id: i32,novo_nome: &str) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;
    
    match mudar_nome_categoria(&mut conn, id, novo_nome){
        Ok(categoria) => {
            let json = serde_json::to_string(&categoria);
            match json{
                Ok(j) => {
                    tauri_create_log(&mut conn, format!("Alterar categoria {} para {}", id, novo_nome))?;
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível alterar categoria: {}",err))}
    }
}
//Editora
#[command]
pub fn salvar_editora_tauri(nome: &str) -> Result<String, String> {
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match criar_editora(&mut conn, nome){
        Ok(editora) => {
            let json = serde_json::to_string(&editora);
            match json{
                Ok(j) => {
                    tauri_create_log(&mut conn, format!("Criar editora {}",nome))?;
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível criar editora: {}",err))}
    }
}
#[command]
pub fn listar_editoras_tauri(nome: &str) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_editoras(&mut conn, nome){
        Ok(editoras) => {
            let json = serde_json::to_string(&editoras);
            match json{
                Ok(j) => Ok(j),
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível listar editoras: {}",err))}
    }
}
#[command]
pub fn apagar_editora_tauri(id: i32) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match apagar_editora(&mut conn, id){
        Ok(editora) => {
            let json = serde_json::to_string(&editora);
            match json{
                Ok(j) => {
                    tauri_create_log(&mut conn, format!("Apagar editora {}",id))?;
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível apagar editora: {}",err))}
    }
}
#[command]
pub fn mudar_nome_editora_tauri(id: i32,novo_nome: &str) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match mudar_nome_editora(&mut conn, id, novo_nome){
        Ok(editora) => {
            let json = serde_json::to_string(&editora);
            match json{
                Ok(j) => {
                    tauri_create_log(&mut conn, format!("Alterarou editora {} para {}",id, novo_nome))?;
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível alterar editora: {}",err))}
    }
}
#[command]
pub fn buscar_editora_id_tauri(id: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match buscar_editora_id(&mut conn, id){
        Ok(editora) => {
            let json = serde_json::to_string(&editora);
            match json{
                Ok(j) => {
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível buscar editora: {}",err))}
    }
}
//Idioma
#[command]
pub fn salvar_idioma_tauri(nome: &str) -> Result<String, String> {
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match criar_idioma(&mut conn, nome){
        Ok(idioma) => {
            let json = serde_json::to_string(&idioma);
            match json{
                Ok(j) => {
                    tauri_create_log(&mut conn, format!("Criar idioma {}",nome))?;    
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível criar idioma: {}",err))}
    }
}
#[command]
pub fn listar_idiomas_tauri(nome: &str) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_idiomas(&mut conn, nome){
        Ok(idiomas) => {
            let json = serde_json::to_string(&idiomas);
            match json{
                Ok(j) => Ok(j),
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível listar idiomas: {}",err))}
    }
}
#[command]
pub fn buscar_idioma_id_tauri(id: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match buscar_idioma_id(&mut conn, id){
        Ok(idioma) => {
            let json = serde_json::to_string(&idioma);
            match json{
                Ok(j) => {   
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível buscar idioma: {}",err))}
    }
}
#[command]
pub fn apagar_idioma_tauri(id: i32) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match apagar_idioma(&mut conn, id){
        Ok(usize) => {
            let json = serde_json::to_string(&usize);
            match json{
                Ok(j) => {
                    tauri_create_log(&mut conn, format!("Apagar idioma {}",id))?;    
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível apagar idioma: {}",err))}
    }
}
#[command]
pub fn mudar_nome_idioma_tauri(id: i32,novo_nome: &str) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match mudar_nome_idioma(&mut conn, id, novo_nome){
        Ok(idioma) => {
            let json = serde_json::to_string(&idioma);
            match json{
                Ok(j) => {
                    tauri_create_log(&mut conn, format!("Mudar idioma {} para {}", id, novo_nome))?;    
                    Ok(j)
                },
                Err(e) => Err(e.to_string()), 
            }
        },
        Err(err) => {Err(format!("Não foi possível alterar idioma: {}",err))}
    }
}
//Livro
#[command]
pub fn salvar_livro_tauri(
   codigo_barras_novo:Option<&str>,
   titulo_novo:&str,
   quantidade_novo:i16,
   paginas_novo:Option<i16>,
   publicacao_novo:Option<i16>,
   editora_id_novo:i32,
   edicao_novo:&str,
   volume_novo:i16,
   idioma_id_novo:i32,
   origem_novo:Option<&str>,
   descricao_novo:Option<&str>,
) -> Result<String, String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match criar_livro(
        &mut conn,
        codigo_barras_novo,
        titulo_novo,
        quantidade_novo,
        paginas_novo,
        publicacao_novo,
        editora_id_novo,
        edicao_novo,
        volume_novo,
        idioma_id_novo,
        origem_novo,
        descricao_novo
    ) {
       Ok(livro) => {
            let json = serde_json::to_string(&livro);
                match json{
                    Ok(j) => {
                        let codigo_msg = match codigo_barras_novo{
                            Some(c) => c.to_string(),
                            None => format!("sem código, com id: {}",livro.id),
                        };
                        tauri_create_log(&mut conn, format!("Criar livro {} - {}", codigo_msg, titulo_novo))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível criar livro: {}",err))}
    }
}

#[command]
pub fn listar_livros_tauri(
    titulo_busca: Option<&str>, 
    codigo_barras_busca: Option<&str>,
    quantidade_busca: Option<i16>,
    paginas_busca: Option<i16>,
    publicacao_busca: Option<i16>,
    editora_id_busca: Option<i32>,
    edicao_busca: Option<&str>,
    volume_busca: Option<i16>,
    idioma_id_busca: Option<i32>,
    origem_busca: Option<&str>,
    descricao_busca: Option<&str>,
) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_livros(
        &mut conn,
        titulo_busca,
        codigo_barras_busca,
        quantidade_busca,
        paginas_busca,
        publicacao_busca,
        editora_id_busca,
        edicao_busca,
        volume_busca,
        idioma_id_busca,
        origem_busca,
        descricao_busca,
    ) {
       Ok(livros) => {
            let json = serde_json::to_string(&livros);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível listar livros: {}",err))}
    }
}
#[command]
pub fn buscar_livro_id_tauri(id: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;
    match buscar_livro_id(
        &mut conn,
        id,
    ) {
       Ok(livro) => {
            let json = serde_json::to_string(&livro);
                match json{
                    Ok(j) => {    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível buscar livro: {}",err))}
    }
}
#[command]
pub fn apagar_livro_tauri(id_apagar: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match apagar_livro(
        &mut conn,
        id_apagar,
    ) {
       Ok(livro) => {
            let json = serde_json::to_string(&livro);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Apagar livro {}", id_apagar))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível apagar livro: {}",err))}
    }
}
#[command]
pub fn alterar_livro_tauri(
    id_alterar: i32,
    titulo_novo: Option<&str>, 
    codigo_barras_novo: Option<&str>,
    quantidade_novo: Option<i16>,
    paginas_novo: Option<i16>,
    publicacao_novo: Option<i16>,
    editora_id_novo: Option<i32>,
    edicao_novo: Option<&str>,
    volume_novo: Option<i16>,
    idioma_id_novo: Option<i32>,
    origem_novo: Option<&str>,
    descricao_novo: Option<&str>,
) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match alterar_livro(
        &mut conn,
        id_alterar,
        titulo_novo,
        codigo_barras_novo,
        quantidade_novo,
        paginas_novo,
        publicacao_novo,
        editora_id_novo,
        edicao_novo,
        volume_novo,
        idioma_id_novo,
        origem_novo,
        descricao_novo,
    ) {
       Ok(livro) => {
            let json = serde_json::to_string(&livro);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Alterar livro {}", id_alterar))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível alterar livro: {}",err))}
    }
}
//Autor
#[command]
pub fn criar_autor_tauri(nome: &str) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match criar_autor(&mut conn, nome){
        Ok(autor) => {
            let json = serde_json::to_string(&autor);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Criar autor {}", nome))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível criar autor: {}",err))}
    }    
}
#[command]
pub fn listar_autores_tauri(nome: &str) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_autores(&mut conn, nome){
        Ok(autores) => {
            let json = serde_json::to_string(&autores);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível listar autoes: {}",err))}
    }    
}
#[command]
pub fn apagar_autor_tauri(id: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match apagar_autor(&mut conn, id){
        Ok(autor) => {
            let json = serde_json::to_string(&autor);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Apagar autor {}", id))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível apagar autor: {}",err))}
    }    
}
#[command]
pub fn alterar_autor_tauri(id: i32, novo_nome: &str) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match mudar_nome_autor(&mut conn, id, novo_nome){
        Ok(autor) => {
            let json = serde_json::to_string(&autor);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Alterar autor {} para {}", id, novo_nome))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível alterar autor: {}",err))}
    }    
}
//Pessoa
fn censor_cpf(cpf: &str) -> String {
    if cpf.len() != 11 {
        // CPF should be exactly 11 characters
        return String::from("Invalid CPF length");
    }

    let mut censored_cpf = String::new();

    // Censor the first 5 digits
    for (index, digit) in cpf.chars().enumerate() {
        if index < 6 {
            censored_cpf.push('*');
        } else {
            censored_cpf.push(digit);
        }
    }

    censored_cpf
}
fn format_cpf(cpf: &str) -> String {
    cpf.chars()
        .filter(|&c| c.is_digit(10)) // Keep only digits
        .collect()
}
#[command]
pub fn criar_pessoa_tauri(
    cpf: &str,
    nome: &str,
    email: Option<&str>,
    celular: Option<&str>,
) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;
    let formated_cpf = format_cpf(cpf);
    match criar_pessoa(&mut conn, &formated_cpf, nome, email, celular){
        Ok(pessoa) => {
            let json = serde_json::to_string(&pessoa);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Criar pessoa {} - {}",censor_cpf(&formated_cpf), nome))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível criar pessoa: {}",err))}
    }
}
#[command]
pub fn listar_pessoas_tauri(
    cpf: Option<&str>,
    nome: Option<&str>,
    email: Option<&str>,
    celular: Option<&str>,
) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;
    let formated_cpf: Option<String> = match cpf{
        Some(cpf) => Some(format_cpf(cpf)),
        None => None,
    };
    match listar_pessoas(&mut conn, formated_cpf.as_deref(), nome, email, celular){
        Ok(pessoas) => {
            let json = serde_json::to_string(&pessoas);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível listar pessoas: {}",err))}
    }
}
#[command]
pub fn apagar_pessoa_tauri( cpf: &str) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match apagar_pessoa(&mut conn, cpf){
        Ok(pessoa) => {
            let json = serde_json::to_string(&pessoa);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Apagar pessoa {}", censor_cpf(&format_cpf(cpf))))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível apagar pessoa: {}",err))}
    }
}
#[command]
pub fn alterar_pessoa_tauri(
    cpf: &str,
    nome: Option<&str>,
    email: Option<&str>,
    celular: Option<&str>,
) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;
    let formated_cpf = format_cpf(cpf);
    match alterar_pessoa(&mut conn, &formated_cpf, nome, email, celular){
        Ok(pessoa) => {
            let json = serde_json::to_string(&pessoa);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Alterar pessoa {}", censor_cpf(&formated_cpf)))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível alterar pessoa: {}",err))}
    } 
}
//AutorLivro
#[command]
pub fn adicionar_autor_livro_tauri(id_autor: i32, id_livro: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match adicionar_autor_livro(&mut conn, id_autor, id_livro){
        Ok(al) => {
            let json = serde_json::to_string(&al);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Adicionou autor {} para o livro {}", id_autor, id_livro))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível adicionar autor ao livro: {}",err))}
    }
}
#[command]
pub fn remover_autor_livro_tauri(id_autor: i32, id_livro: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match remover_autor_livro(&mut conn, id_autor, id_livro){
        Ok(al) => {
            let json = serde_json::to_string(&al);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Removeu autor {} do livro {}", id_autor, id_livro))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível remover autor do livro: {}",err))}
    }
}
#[command]
pub fn listar_livros_autor_tauri(id_autor: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_livros_autor(&mut conn, id_autor){
        Ok(al) => {
            let json = serde_json::to_string(&al);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível procurar livros do autor: {}",err))}
    }
}
#[command]
pub fn listar_autores_livro_tauri(id_livro: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_autores_livro(&mut conn, id_livro){
        Ok(al) => {
            let json = serde_json::to_string(&al);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível procurar autores do livro: {}",err))}
    }
}
//CategoriaLivro
#[command]
pub fn adicionar_categoria_livro_tauri(id_categoria: i32, id_livro: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match adicionar_categoria_livro(&mut conn, id_categoria, id_livro){
        Ok(al) => {
            let json = serde_json::to_string(&al);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Adicionou categoria {} para o livro {}", id_categoria, id_livro))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível adcionar categoria ao livro: {}",err))}
    }
}
#[command]
pub fn remover_categoria_livro_tauri(id_categoria: i32, id_livro: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match remover_categoria_livro(&mut conn, id_categoria, id_livro){
        Ok(al) => {
            let json = serde_json::to_string(&al);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Removeu categoria {} do livro {}", id_categoria, id_livro))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível remover categoria do livro: {}",err))}
    }
}
#[command]
pub fn listar_livros_categoria_tauri(id_categoria: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_livros_categoria(&mut conn, id_categoria){
        Ok(al) => {
            let json = serde_json::to_string(&al);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível procurar livros da categoria: {}",err))}
    }
}
#[command]
pub fn listar_categorias_livro_tauri(id_livro: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_categorias_livro(&mut conn, id_livro){
        Ok(al) => {
            let json = serde_json::to_string(&al);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível procurar categorias do livro: {}",err))}
    }
}
//Emprestimo
#[command]
pub fn criar_emprestimo_tauri( 
    livro_id: i32,
    pessoa_cpf: &str,
    comeco_iso_date: &str,
    fim_iso_date: Option<&str>
) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    let comeco = NaiveDate::parse_from_str(&comeco_iso_date, "%Y-%m-%d")
        .map_err(|e| format!("Comeco invalido: {}", e))?;
    
    let fim = parse_date_str_option(fim_iso_date).map_err(|e| e.to_string())?;
    let formated_cpf = format_cpf(pessoa_cpf);
    match criar_emprestimo(&mut conn, livro_id, &formated_cpf, comeco, fim){
        Ok(emprestimo) => {
            let json = serde_json::to_string(&emprestimo);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Criar emprestimo do livro {} para {}", livro_id, censor_cpf(&formated_cpf)))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível criar empréstimo: {}",err))}
    }
}
#[command]
pub fn listar_emprestimos_tauri(
    id_busca: Option<i32>,
    livro_id_busca: Option<i32>,
    pessoa_cpf_busca: Option<&str>,
    comeco_iso_str_busca: Option<&str>, 
    fim_iso_str_busca: Option<&str>,
    cancelado_busca: Option<bool>, 
) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    let comeco_busca = parse_date_str_option(comeco_iso_str_busca).map_err(|e| e.to_string())?;

    let fim_busca = parse_date_str_option(fim_iso_str_busca).map_err(|e| e.to_string())?;

    match listar_emprestimos(&mut conn, id_busca, livro_id_busca, pessoa_cpf_busca, comeco_busca, fim_busca, cancelado_busca) {
        Ok(emprestimos) => {
            let json = serde_json::to_string(&emprestimos);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível listar empréstimos: {}",err))}
    }
}
#[command]
pub fn cancelar_emprestimo_tauri(id_cancelar: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match cancelar_emprestimo(&mut conn, id_cancelar){
        Ok(emprestimo) => {
            let json = serde_json::to_string(&emprestimo);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Cancelar emprestimo {}", id_cancelar))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível cancelar empréstimo: {}",err))}
    }

}
//Usuario 
#[command]
pub fn criar_usuario_tauri(nome_criar: &str, senha_criar: &str) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match criar_usuario(&mut conn, nome_criar, hash(senha_criar)){
        Ok(usuario) => {
            let json = serde_json::to_string(&usuario);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Criar usuário {} com id {}", nome_criar, usuario.id))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível criar usuário: {}",err))}
    }
}
#[command]
pub fn apagar_usuario_tauri(id: i32) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match apagar_usuario(&mut conn, id){
        Ok(usuario) => {
            let json = serde_json::to_string(&usuario);
                match json{
                    Ok(j) => {
                        tauri_create_log(&mut conn, format!("Apagar usuário {}", id))?;    
                        Ok(j)
                    },
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível apagar usuário: {}",err))}
    }
}
#[command]
pub fn listar_usuarios_tauri(filtro: &str) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    match listar_usuarios(&mut conn, filtro){
        Ok(usuarios) => {
            let json = serde_json::to_string(&usuarios);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        } 
        Err(err) => {Err(format!("Não foi possível byscar usuários: {}",err))}
    }
}

fn hash(string: &str) -> String{
    let mut hasher = Sha256::new();
    hasher.update(string);
    let result = hasher.finalize();
    format!("{:x}", result)
}
#[command]
pub fn logar_tauri(nome: &str, senha: &str) -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;

    let user = buscar_usuario_exato(&mut conn, nome).expect("Usuário não encontrado");
    if user.hashed_senha == hash(senha) {
        env::set_var("USER", nome);
        Ok("Login com sucesso".to_string())
    }
    else{
        Err("Senha inválida!".to_string())
    }
}

pub fn garantir_existencia_adm() -> Result<String,String>{
    let mut conn: PgConnection = establish_connection().map_err(|e| format!("Erro na conexão: {}",e) )?;
    match buscar_usuario_exato(&mut conn, "admin"){
        Ok(u) => {
                let json = serde_json::to_string(&u);
                match json{
                    Ok(j) => Ok(j),
                    Err(e) => Err(e.to_string()), 
                }
        },
        Err(_e) => {
            match criar_usuario(&mut conn, "admin", hash("12345678")) {
                Ok(u) => {
                    let json = serde_json::to_string(&u);
                    match json{
                        Ok(j) =>{
                            tauri_create_log(&mut conn, "Criado admin original".to_string())?;    
                            Ok(j)
                        },
                        Err(e) => Err(e.to_string()), 
                    } 
                },
                Err(e) => Err(e.to_string()),
            }
        }
    }
}