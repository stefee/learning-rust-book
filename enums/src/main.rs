fn main() {
    l61();
    l61b();
    l62();
    l64();
    l65();
    l66();
}

fn l61() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("The home IP address is {} kind {:?}.", home.address, home.kind);
    println!("The loopback IP address is {} kind {:?}.", loopback.address, loopback.kind);
}

fn l61b() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("The home IP address is {:?}.", home);
    println!("The loopback IP address is {:?}.", loopback);
}

fn l62() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("The message is {:?}.", self);
        }
    }

    let q = Message::Quit;
    let m = Message::Move { x: 20, y: 30 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(1, 2, 3);

    q.call();
    m.call();
    w.call();
    c.call();
}

fn l64() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    let penny = value_in_cents(Coin::Penny);
    let nickel = value_in_cents(Coin::Nickel);
    let dime = value_in_cents(Coin::Dime);
    let quarter_alabama = value_in_cents(Coin::Quarter(UsState::Alabama));
    let quarter_alaska = value_in_cents(Coin::Quarter(UsState::Alaska));

    println!("Penny is {}.", penny);
    println!("Nickel is {}.", nickel);
    println!("Dime is {}.", dime);
    println!("Quarter in Alabama is {}.", quarter_alabama);
    println!("Quarter in Alaska is {}.", quarter_alaska);
}

fn l65() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1)
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Five is {:?}.", five);
    println!("Six is {:?}.", six);
    println!("None is {:?}.", none);
}

fn l66() {
    let some_u8_value = Some(3);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }

    if let Some(2) = some_u8_value {
        println!("two");
    } else {
        println!("not two");
    }
}
