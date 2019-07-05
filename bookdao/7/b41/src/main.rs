// 代表信件
#[derive(Clone,Debug)]
pub struct Letter {
    text: String,
}

// 代表信封，Option 标识信封中是否有信件
#[derive(Clone)]
pub struct Envelope {
    letter: Option<Letter>,
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
impl Envelope {
    pub fn wrap(&mut self, letter: &Letter) {
        self.letter = Some(letter.clone());
    }
}

// 购买待邮戳的新信封
pub fn buy_prestamped_envelope() -> Envelope {
    Envelope{letter: None}
}

impl PickupLorryHandle {
    // 装信
    pub fn pickup(&mut self, envelope: &Envelope) {
        /* give letter */
    }
    // 送信
    pub fn done(&mut self) {
        self.done = true;
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
    envelope.wrap(&letter);

    let mut lorry = order_pickup();
    lorry.pickup(&envelope);
    lorry.done();
}
