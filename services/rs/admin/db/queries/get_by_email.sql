SELECT *
FROM admins
WHERE email = $1 AND NOT deleted