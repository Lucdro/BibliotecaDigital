use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::result::Error;
use bibliotecadigital::schema::logs::dsl::*;
use bibliotecadigital::models::Log;
use bibliotecadigital::schema::logs;

pub fn criar_log(conn: &mut PgConnection,
    descricao_nova: String
) -> Result<Log,Error>{
    let new_log = Log {
        id: 0,
        descricao: descricao_nova,
        criado_em: None,
    };
    diesel::insert_into(logs::table)
        .values(&new_log)
        .returning(Log::as_returning())
        .get_result(conn)
}

pub fn listar_logs(conn: &mut PgConnection,
    filtro: &str,
) -> Result<Vec<Log>,Error>{
    logs.filter(descricao.ilike(format!("%{}%",filtro)))
    .get_results(conn)
}
