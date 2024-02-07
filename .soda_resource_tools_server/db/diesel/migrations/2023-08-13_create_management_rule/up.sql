CREATE TABLE table_management_rule
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    src          VARCHAR NOT NULL,
    target       VARCHAR NOT NULL,
    content_type VARCHAR NOT NULL,
    mode         VARCHAR NOT NULL,
    period       VARCHAR NOT NULL,
    status       VARCHAR NOT NULL,
    monitor      VARCHAR NOT NULL
);
