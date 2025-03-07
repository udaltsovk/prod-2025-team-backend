UPDATE admins
SET email                = '',
    password_hash        = '',
    last_password_change = current_timestamp,
    deleted              = true
WHERE id = $1 AND NOT deleted
RETURNING *
