INSERT INTO admins (id, email, password_hash, last_password_change, deleted)
VALUES ($1, $2, $3, current_timestamp, false)
RETURNING *
