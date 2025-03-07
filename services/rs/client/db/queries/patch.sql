UPDATE clients
SET name                 = COALESCE($2, name),
    surname              = COALESCE($3, surname),
    patronymic           = COALESCE($4, patronymic),
    email                = COALESCE($5, email),
    password_hash        = COALESCE($6, password_hash),
    last_password_change = COALESCE($7, last_password_change),
    send_notifications   = COALESCE($8, send_notifications),
    is_internal          = COALESCE($9, is_internal),
    verified             = COALESCE($10, verified)
WHERE id = $1 AND NOT DELETED
RETURNING *
