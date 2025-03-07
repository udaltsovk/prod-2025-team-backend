UPDATE admins
SET email                = COALESCE($2, email),
    password_hash        = COALESCE($3, password_hash),
    last_password_change = COALESCE($4, last_password_change)
WHERE id = $1 AND NOT deleted
RETURNING *
