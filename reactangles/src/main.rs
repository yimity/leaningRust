fn main() {
    let width = 30;
    let height = 50;

    let rect = (30, 50);

    let reactangle = Reactangle {
        width: 30,
        height: 50,
    };

    // let result = area1(width, height);

    // let result = area2(rect);

    let result = area3(&reactangle);

    println!("The area of the rectangle is {} square pixels.", result);
}

struct Reactangle {
    width: u32,
    height: u32,
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(wh: (u32, u32)) -> u32 {
    return wh.0 * wh.1;
}

fn area3(reactangle: &Reactangle) -> u32 {
    return reactangle.width * reactangle.height;
}