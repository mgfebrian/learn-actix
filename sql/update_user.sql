UPDATE testing.users
SET email      = $1,
    first_name = $2,
    last_name  = $3,
    username   = $4
WHERE email = $1
RETURNING $table_fields;