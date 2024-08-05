use std::{fs::{self, File}, io::Read, process::Output, time::{self, Instant, SystemTime, UNIX_EPOCH}};
use rusqlite::{Connection, Result};

struct LutrisGame
{
    name: String, 
    sort_name: String,
    last_played: Instant,
    install_time: Instant,

}

pub fn get_database_file() -> Connection
{
    Connection::open("pga.db").unwrap()
}

pub fn modify_database(game: &LutrisGame)
{
    fn instant_to_u64(i: Instant) -> u64
    {
        (SystemTime::now() - i.elapsed())
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
    }
    let lp = instant_to_u64(game.last_played).to_string();

    let it = instant_to_u64(game.install_time).to_string();
    
    let connection = get_database_file();
    let _ = connection.execute("INSERT INTO games (name, sortname, platform, runner, lastplayed, installed, configpath) VALUES(?1,'Windows','wine',?2,1,?3,'?4')",
    [game.name.clone(), game.sort_name.clone(), lp, it.clone(), format!("{}-{}", game.name, it.clone())]);
}

pub fn add_config()
{

}

//INSERT INTO games VALUES(10,'Battle Brothers','battle-brothers',NULL,NULL,'Windows','wine',NULL,'',NULL,1718355309,1,1710490978,NULL,'battle-brothers-1710490978',0,0,0,34.1473358889411215,0,NULL,NULL,NULL,'');
/*CREATE TABLE games (
id INTEGER PRIMARY KEY, 
name TEXT, 
sortname TEXT, 
slug TEXT, 
installer_slug TEXT, 
parent_slug TEXT, 
platform TEXT, 
runner TEXT, 
executable TEXT, 
directory TEXT, 
updated DATETIME, 
lastplayed INTEGER, 
installed INTEGER, 
installed_at INTEGER, 
year INTEGER, 
configpath TEXT, 
has_custom_banner INTEGER, 
has_custom_icon INTEGER, 
has_custom_coverart_big INTEGER, 
playtime REAL, 
service TEXT, 
service_id TEXT, 
discord_id TEXT);

*/