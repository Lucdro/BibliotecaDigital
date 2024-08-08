-- Your SQL goes here
CREATE TABLE "categorias"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"nome" VARCHAR(100) NOT NULL
);

CREATE TABLE "pessoas"(
	"cpf" VARCHAR(100) NOT NULL PRIMARY KEY,
	"nome" VARCHAR(100) NOT NULL,
	"email" VARCHAR(100),
	"celular" VARCHAR(100)
);

CREATE TABLE "idiomas"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"nome" VARCHAR(100) NOT NULL
);

CREATE TABLE "editoras"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"nome" VARCHAR(100) NOT NULL
);

CREATE TABLE "livros" (
    "id" SERIAL NOT NULL PRIMARY KEY,
    "codigo_barras" VARCHAR(100),
    "titulo" VARCHAR(100) NOT NULL,
    "quantidade" INT2 NOT NULL,
    "paginas" INT2,
    "publicacao" INT2,
    "editora_id" INT4,
    "edicao" VARCHAR(100) NOT NULL,
    "volume" INT2 NOT NULL,
    "idioma_id" INT4,
    "origem" VARCHAR(100),
    "descricao" VARCHAR(100),
    FOREIGN KEY ("editora_id") REFERENCES "editoras"("id") ON DELETE SET NULL,
    FOREIGN KEY ("idioma_id") REFERENCES "idiomas"("id") ON DELETE SET NULL
);

CREATE TABLE "autores"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"nome" VARCHAR(100) NOT NULL
);

CREATE TABLE "categorias_livro"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"livro_id" INT4,
	"categoria_id" INT4,
	FOREIGN KEY ("livro_id") REFERENCES "livros"("id") ON DELETE SET NULL,
	FOREIGN KEY ("categoria_id") REFERENCES "categorias"("id") ON DELETE SET NULL
);

CREATE TABLE "autores_livro"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"livro_id" INT4,
	"autor_id" INT4,
	FOREIGN KEY ("livro_id") REFERENCES "livros"("id") ON DELETE SET NULL,
	FOREIGN KEY ("autor_id") REFERENCES "autores"("id") ON DELETE SET NULL
);

CREATE TABLE "emprestimos"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"pessoa_cpf" VARCHAR(100),
	"livro_id" INT4,
	"comeco" DATE NOT NULL,
	"fim" DATE,
	"cancelado" BOOL NOT NULL,
	FOREIGN KEY ("pessoa_cpf") REFERENCES "pessoas"("cpf") ON DELETE SET NULL,
	FOREIGN KEY ("livro_id") REFERENCES "livros"("id") ON DELETE SET NULL
);

