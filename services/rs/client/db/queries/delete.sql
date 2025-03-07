UPDATE clients
SET name                 = '',
    surname              = '',
    patronymic           = '',
    email                = '',
    password_hash        = '',
    last_password_change = current_timestamp,
    send_notifications   = false,
    deleted              = true
WHERE id = $1 AND NOT deleted
RETURNING *
