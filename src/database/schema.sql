PRAGMA foreign_keys=ON;

-- config, used to store version and hash algorithm.
CREATE TABLE IF NOT EXISTS config(
    name TEXT NOT NULL,
    value BLOB NOT NULL,
    PRIMARY KEY (name)
);

-- files, stored by hash
CREATE TABLE IF NOT EXISTS files(
    id INTEGER NOT NULL,
    hash BLOB NOT NULL,
    PRIMARY KEY(id),
    UNIQUE (hash)
);

CREATE TABLE IF NOT EXISTS tag_names(
    id INTEGER NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY(id),
    UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS tag_values(
    id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL REFERENCES tag_names(id),
    value TEXT NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (tag_id, value)
);

CREATE INDEX tag_values_by_tag ON tag_values(tag_id);

CREATE VIEW IF NOT EXISTS tags AS
    SELECT
        tag_names.id as name_id,
        tag_names.name as name,
        tag_values.id as value_id,
        tag_values.value as value
    FROM tag_values
    JOIN tag_names ON tag_names.id = tag_values.tag_id;

CREATE TRIGGER tags_insert
INSTEAD OF INSERT ON tags
BEGIN
    -- create tag name if it does not exist
    INSERT OR IGNORE INTO tag_names(name)
        VALUES (NEW.name);

    -- create tag value
    INSERT INTO tag_values(tag_id, value)
        VALUES (
            (SELECT id FROM tag_names WHERE name = NEW.name),
            NEW.value
        );
END;

CREATE TRIGGER tags_delete
INSTEAD OF DELETE ON tags
FOR EACH ROW
BEGIN
    -- delete tag value
    DELETE FROM tag_values
        WHERE tag_id = OLD.name_id
        AND value = OLD.value;

    -- delete tag name if no tag values left
    DELETE FROM tag_names
    WHERE (
        SELECT count(id)
            FROM tag_values
            WHERE tag_id = tag_names.id
        ) = 0;
END;

CREATE TABLE IF NOT EXISTS file_tag_values(
    file_id INTEGER NOT NULL REFERENCES files(id),
    tag_value_id INTEGER NOT NULL REFERENCES tag_values(id),
    PRIMARY KEY (file_id, tag_value_id)
);

CREATE INDEX file_tag_values_by_file ON file_tag_values(file_id);
CREATE INDEX file_tag_values_by_tag_value ON file_tag_values(tag_value_id);

CREATE VIEW IF NOT EXISTS file_tags AS
    SELECT
        files.id as file_id,
        files.hash as hash,
        tags.name_id as name_id,
        tags.name as name,
        tags.value_id as value_id,
        tags.value as value
    FROM file_tag_values
    JOIN files ON file_tag_values.file_id = files.id
    JOIN tags ON file_tag_values.tag_value_id = tags.value_id;

CREATE TRIGGER file_tags_insert
INSTEAD OF INSERT ON file_tags
BEGIN
    INSERT INTO file_tag_values(file_id, tag_value_id)
    VALUES (
        (SELECT id FROM files WHERE hash = NEW.hash),
        (SELECT value_id FROM tags WHERE name = NEW.name AND value = NEW.value)
    );
END;

CREATE TRIGGER file_tags_delete
INSTEAD OF DELETE ON file_tags
FOR EACH ROW
BEGIN
    DELETE FROM file_tag_values
    WHERE file_id = OLD.file_id
    AND tag_value_id = OLD.value_id;
END;
