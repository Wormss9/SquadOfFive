mod database;
use database::create_database_connection;
use database::Player;

fn main() {
    let mut client = create_database_connection();

    match Player::create(&mut client, "Jozko", "password"){
        Ok(_) => (),
        Err(err) => print!("{:?}",err.as_db_error())
    };
    let x = Player::get(&mut client, "Jozko").expect("Failed to get player");
    print!("{:?}", x)
}
