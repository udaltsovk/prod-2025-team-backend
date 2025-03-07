-- +goose Up
-- +goose StatementBegin
SELECT 'up SQL query';
CREATE TABLE seats
(
    id       uuid PRIMARY KEY NOT NULL,
    number   int              NOT NULL UNIQUE,
    type     text             NOT NULL,
    capacity int              NOT NULL,
    features text[]           NOT NULL,
    cost     int              NOT NULL
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
DROP TABLE seats;
-- +goose StatementEnd
