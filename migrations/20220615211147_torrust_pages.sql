CREATE TABLE IF NOT EXISTS torrust_pages (
    page_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    route TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    creation_date INT(10) NOT NULL
)
