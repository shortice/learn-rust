enum Danila {
    Norm,
    NeNorm,
}

impl Danila {
    fn print(&self) {
        let kavo = match self {
            Danila::NeNorm => "Nafig",
            Danila::Norm => "Eeeee",
        };

        println!("{}", kavo);
    }
}

pub fn enum_rustics() {
    let norm = Danila::Norm;
    let nenorm = Danila::NeNorm;

    norm.print();
    nenorm.print();
}
