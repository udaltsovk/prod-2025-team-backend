-- +goose Up
-- +goose StatementBegin
SELECT 'up SQL query';
ALTER TABLE seats DROP CONSTRAINT seats_number_key;
ALTER TABLE seats ADD UNIQUE (number, type);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
ALTER TABLE seats DROP CONSTRAINT seats_number_type_key;
ALTER TABLE seats ADD UNIQUE (number);
-- +goose StatementEnd
