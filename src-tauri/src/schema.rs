// @generated automatically by Diesel CLI.

diesel::table! {
    autores (id) {
        id -> Int4,
        #[max_length = 100]
        nome -> Varchar,
    }
}

diesel::table! {
    autores_livro (id) {
        id -> Int4,
        livro_id -> Nullable<Int4>,
        autor_id -> Nullable<Int4>,
    }
}

diesel::table! {
    categorias (id) {
        id -> Int4,
        #[max_length = 100]
        nome -> Varchar,
    }
}

diesel::table! {
    categorias_livro (id) {
        id -> Int4,
        livro_id -> Nullable<Int4>,
        categoria_id -> Nullable<Int4>,
    }
}

diesel::table! {
    editoras (id) {
        id -> Int4,
        #[max_length = 100]
        nome -> Varchar,
    }
}

diesel::table! {
    emprestimos (id) {
        id -> Int4,
        #[max_length = 100]
        pessoa_cpf -> Nullable<Varchar>,
        livro_id -> Nullable<Int4>,
        comeco -> Date,
        fim -> Nullable<Date>,
        cancelado -> Bool,
    }
}

diesel::table! {
    idiomas (id) {
        id -> Int4,
        #[max_length = 100]
        nome -> Varchar,
    }
}

diesel::table! {
    livros (id) {
        id -> Int4,
        #[max_length = 100]
        codigo_barras -> Nullable<Varchar>,
        #[max_length = 100]
        titulo -> Varchar,
        quantidade -> Int2,
        paginas -> Nullable<Int2>,
        publicacao -> Nullable<Int2>,
        editora_id -> Nullable<Int4>,
        #[max_length = 100]
        edicao -> Varchar,
        volume -> Int2,
        idioma_id -> Nullable<Int4>,
        #[max_length = 100]
        origem -> Nullable<Varchar>,
        #[max_length = 100]
        descricao -> Nullable<Varchar>,
    }
}

diesel::table! {
    logs (id) {
        id -> Int4,
        #[max_length = 200]
        descricao -> Varchar,
        criado_em -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pessoas (cpf) {
        #[max_length = 100]
        cpf -> Varchar,
        #[max_length = 100]
        nome -> Varchar,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        #[max_length = 100]
        celular -> Nullable<Varchar>,
    }
}

diesel::table! {
    usuarios (id) {
        id -> Int4,
        #[max_length = 100]
        nome -> Varchar,
        #[max_length = 100]
        hashed_senha -> Varchar,
        criado_em -> Nullable<Timestamp>,
    }
}

diesel::joinable!(autores_livro -> autores (autor_id));
diesel::joinable!(autores_livro -> livros (livro_id));
diesel::joinable!(categorias_livro -> categorias (categoria_id));
diesel::joinable!(categorias_livro -> livros (livro_id));
diesel::joinable!(emprestimos -> livros (livro_id));
diesel::joinable!(emprestimos -> pessoas (pessoa_cpf));
diesel::joinable!(livros -> editoras (editora_id));
diesel::joinable!(livros -> idiomas (idioma_id));

diesel::allow_tables_to_appear_in_same_query!(
    autores,
    autores_livro,
    categorias,
    categorias_livro,
    editoras,
    emprestimos,
    idiomas,
    livros,
    logs,
    pessoas,
    usuarios,
);
