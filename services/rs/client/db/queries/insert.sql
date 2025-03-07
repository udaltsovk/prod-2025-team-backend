INSERT INTO clients (id, name, surname, patronymic, email, password_hash, last_password_change, send_notifications, is_internal, verified, deleted)
VALUES ($1, $2, $3, $4, $5, $6, current_timestamp, $7, $8, $9, false)
RETURNING *
