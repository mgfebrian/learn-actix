DELETE FROM testing.users
WHERE username = $1
RETURNING $table_fields;