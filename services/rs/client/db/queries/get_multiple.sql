SELECT *
FROM clients
WHERE id = ANY($1::uuid[]) AND NOT deleted