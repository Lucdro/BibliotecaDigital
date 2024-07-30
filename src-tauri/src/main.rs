// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod tauri_commands;
pub mod models_lib;

use crate::tauri_commands::*;

fn main() {
    println!("{:?}", garantir_existencia_adm());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            salvar_categoria_tauri,
            listar_categorias_tauri,
            apagar_categoria_tauri,
            mudar_nome_categoria_tauri,
            salvar_editora_tauri,
            listar_editoras_tauri,
            buscar_editora_id_tauri,
            apagar_editora_tauri,
            mudar_nome_editora_tauri,
            salvar_idioma_tauri,
            listar_idiomas_tauri,
            buscar_idioma_id_tauri,
            apagar_idioma_tauri,
            mudar_nome_idioma_tauri,
            salvar_livro_tauri,
            listar_livros_tauri,
            buscar_livro_id_tauri,
            apagar_livro_tauri,
            alterar_livro_tauri,
            criar_autor_tauri,
            listar_autores_tauri,
            apagar_autor_tauri,
            alterar_autor_tauri,
            criar_pessoa_tauri,
            apagar_pessoa_tauri,
            listar_pessoas_tauri,
            alterar_pessoa_tauri,
            adicionar_autor_livro_tauri,
            remover_autor_livro_tauri,
            listar_livros_autor_tauri,
            listar_autores_livro_tauri,
            adicionar_categoria_livro_tauri,
            remover_categoria_livro_tauri,
            listar_categorias_livro_tauri,
            listar_livros_categoria_tauri,
            criar_emprestimo_tauri,
            listar_emprestimos_tauri,
            cancelar_emprestimo_tauri,
            criar_usuario_tauri,
            apagar_usuario_tauri,
            listar_usuarios_tauri,
            logar_tauri,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
