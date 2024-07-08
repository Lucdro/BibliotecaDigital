-- Your SQL goes here
CREATE TABLE "usuarios" (
    "id" SERIAL NOT NULL PRIMARY KEY,
    "nome" VARCHAR(100) UNIQUE NOT NULL,
    "hashed_senha" VARCHAR(100) NOT NULL,
    "criado_em" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);