use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let email = std::env::var("email").expect("email must be set.");
    let password = std::env::var("password").expect("password must be set.");

    println!("Usernam de: {email} \nPassword: {password}");

    let account = MinecraftAccount {
        email: email,
        password: password,
        account_type: AccountType::MICROSOFT,
        server: String::from("127.0.0.1"),
        port: 25565,
    };
    println!("{:?}", account);
}


#[derive(Debug)]
struct MinecraftAccount {
    email: String,
    password: String,
    account_type: AccountType,
    server: String,
    port: u32,
}
#[derive(Debug)]
enum AccountType{
    MOJANG,
    MICROSOFT,
}

fn connect(acc: MinecraftAccount) {
    let version_num = 763; // 1.20.1 version handlelr"
}
