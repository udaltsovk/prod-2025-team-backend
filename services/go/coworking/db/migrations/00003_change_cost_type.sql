-- +goose Up
-- +goose StatementBegin
SELECT 'up SQL query';
ALTER TABLE seats DROP COLUMN cost;
ALTER TABLE seats ADD COLUMN cost float NOT NULL;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
ALTER TABLE seats DROP COLUMN cost;
ALTER TABLE seats ADD COLUMN cost int NOT NULL;
-- +goose StatementEnd
