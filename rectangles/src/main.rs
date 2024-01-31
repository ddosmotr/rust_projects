#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// объявление методов
impl Rect {
    // связанная функция (не принимает экземпляр)
    fn square(size: u32) -> Rect {
        Rect { width: size, height: size }
    }
    // self как любой другой параетр следует правилам владения self/&self/&mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // сопоставление аргументов параметрам игнорирует первый параметр self
    fn can_hold(&self, rect: &Rect) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {

    let r = Rect {
        width: 30,
        height: 50,
    };

    let r2 = Rect {
        width: 30,
        height: 60,
    };

    println!("(func) Площадь прямоугольника равна {} квадратным пикселям", area(&r));
    println!("(method) Площадь прямоугольника равна {} квадратным пикселям", r.area());

    println!("Может ли rect1 содержать в себе rect2? {}", r.can_hold(&r2));

    let r3 = Rect::square(12);

    println!("Квадрат, созданный через связанную функцию {:#?}", r3);

    println!("{:#?}", r)
}

fn area(r: &Rect) -> u32 {
    r.width * r.height
}
