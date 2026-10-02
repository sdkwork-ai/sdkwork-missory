# Postgres migrations

Post-GA schema changes only; the initialization baseline lives in
`../../ddl/baseline/postgres/`. Each migration carries the standard
`-- sdkwork:migration` header and a matching .down.sql.
