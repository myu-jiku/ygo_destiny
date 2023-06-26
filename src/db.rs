use std::error::Error;

mod urls {
    pub const API_CARDINFO: &str = "https://db.ygoprodeck.com/api/v7/cardinfo.php";
    pub const API_CARDSETS: &str = "https://db.ygoprodeck.com/api/v7/cardsets.php";
}

pub fn update(db: &rusqlite::Connection) -> Result<(), Box<dyn Error>> {
    let json_string = reqwest::blocking::get(urls::API_CARDSETS)?
        .text()?
        .replace('\'', "''");

    db.execute("DROP TABLE IF EXISTS sets", ())?;
    db.execute(
        "CREATE TABLE sets (
            name    TEXT PRIMARY KEY,
            date    TEXT,
            code    TEXT,
            cards   INTEGER
        )",
        (),
    )?;

    let sql_columns = [
        ("set_name", "name"),
        ("tcg_date", "date"),
        ("set_code", "code"),
        ("num_of_cards", "cards"),
    ]
    .iter()
    .fold("".to_string(), |mut prev, new| {
        let prefix = if prev.is_empty() { "" } else { ", " };
        prev.push_str(&format!(
            "{prefix}json_extract(value, '$.{}') as {}",
            new.0, new.1
        ));
        prev
    });

    db.execute(
        &format!(
            "INSERT INTO sets SELECT {} FROM json_each('{}')",
            sql_columns, json_string,
        ),
        (),
    )?;

    let json_string = reqwest::blocking::get(urls::API_CARDINFO)?
        .text()?
        .replace('\'', "''");

    db.execute("DROP TABLE IF EXISTS cards", ())?;
    db.execute(
        "CREATE TABLE cards (
            id          INTEGER PRIMARY KEY,
            name        TEXT,
            card_type   TEXT,
            description TEXT,
            atk         INTEGER,
            def         INTEGER,
            level       INTEGER,
            type        TEXT,
            attribute   TEXT,
            archetype   TEXT,
            pend_scale  INTEGER,
            link_rating INTEGER
        )",
        (),
    )?;

    let sql_columns = [
        ("id", "id"),
        ("name", "name"),
        ("type", "card_type"),
        ("desc", "description"),
        ("atk", "atk"),
        ("def", "def"),
        ("level", "level"),
        ("race", "type"),
        ("attribute", "attribute"),
        ("archetype", "archetype"),
        ("scale", "pend_scale"),
        ("linkval", "link_rating"),
    ]
    .iter()
    .fold("".to_string(), |mut prev, new| {
        let prefix = if prev.is_empty() { "" } else { ", " };
        prev.push_str(&format!(
            "{prefix}json_extract(value, '$.{}') as {}",
            new.0, new.1
        ));
        prev
    });

    db.execute(
        &format!(
            "INSERT INTO cards SELECT {} FROM json_each('{}', '$.data')",
            sql_columns, json_string,
        ),
        (),
    )?;

    db.execute("DROP TABLE IF EXISTS set_contents", ())?;
    db.execute(
        "CREATE TABLE set_contents (
            card_id     INTEGER REFERENCES cards(id),
            set_name    TEXT,
            rarity      TEXT
        )",
        (),
    )?;
    db.execute(
        &format!(
            "INSERT INTO set_contents SELECT card_id,
                    json_extract(value, '$.set_name') as set_name,
                    json_extract(value, '$.set_rarity') as rarity
            FROM (
                SELECT  json_extract(value, '$.id') as card_id,
                        json_extract(value, '$.card_sets') as sets
                FROM json_each('{}', '$.data')
            ) as flat, json_each(flat.sets)",
            json_string,
        ),
        (),
    )?;

    Ok(())
}
