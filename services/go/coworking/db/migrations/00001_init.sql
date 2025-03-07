-- +goose Up
-- +goose StatementBegin
SELECT 'up SQL query';
CREATE TABLE coworkings
(
    id      uuid PRIMARY KEY NOT NULL,
    name    text             NOT NULL,
    address text             NOT NULL
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
DROP TABLE coworkings
-- +goose StatementEnd
