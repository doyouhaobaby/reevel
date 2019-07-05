// 代表信件
#[derive(Clone,Debug)]
pub struct Letter {
    text: String,
}

pub struct EmptyEnvelope {
}

pub struct ClosedEnvelope {
    letter: Letter,
}

// 表示信件被装车送走
pub struct PickupLorryHandle {
    done: bool,
}

// 写信
impl Letter {
    pub fn new(text: String) -> Self {
        Letter{text: text}
    }
}

// 装信
impl EmptyEnvelope {
    pub fn wrap(self, letter: Letter) -> ClosedEnvelope {
        ClosedEnvelope{letter: letter}
    }
}

// 购买待邮戳的新信封
pub fn buy_prestamped_envelope() -> EmptyEnvelope {
    EmptyEnvelope{}
}

impl PickupLorryHandle {
    // 装信
    pub fn pickup(&mut self, envelope: ClosedEnvelope) {
        /* give letter */
    }
    // 送信
    pub fn done() {
    }
}

impl Drop for PickupLorryHandle {
    fn drop(&mut self) {
        println!("send");
    }
}

// 信封下单装车
pub fn order_pickup() -> PickupLorryHandle {
    PickupLorryHandle {done:false, }
}

fn main() {
    let letter = Letter::new(String::from("hello rust"));
    //dbg!(letter);

    let mut envelope = buy_prestamped_envelope();
    let closedEnvelope = envelope.wrap(letter);

    let mut lorry = order_pickup();
    lorry.pickup(closedEnvelope);
}
