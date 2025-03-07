SELECT *
FROM clients
WHERE email = $1 AND NOT deleted