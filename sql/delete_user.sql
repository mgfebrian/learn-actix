DELETE FROM testing.users
WHERE email = $1
RETURNING $table_fields;