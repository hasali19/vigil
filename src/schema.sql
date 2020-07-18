CREATE TABLE IF NOT EXISTS hosts (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    ip_address  TEXT    NOT NULL,
    mac_address TEXT    NOT NULL
);
