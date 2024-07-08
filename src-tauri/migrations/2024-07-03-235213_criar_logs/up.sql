-- Your SQL goes here
CREATE TABLE "logs" (
    "id" SERIAL NOT NULL PRIMARY KEY,
    "descricao" VARCHAR(200) NOT NULL,
    "criado_em" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);