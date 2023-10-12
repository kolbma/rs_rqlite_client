CREATE TABLE IF NOT EXISTS test_migration_2 (\
    id INTEGER, \
    val TEXT\
);

INSERT INTO test_migration_2 (id , val) VALUES (1, "test_migration");
