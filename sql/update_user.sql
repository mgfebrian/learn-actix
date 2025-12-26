UPDATE testing.users
SET email      = $2,
    first_name = $3,
    last_name  = $4,
    username   = $5
WHERE username = $1
RETURNING $table_fields;