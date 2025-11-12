use std::io;

// struct Cars {
//     cars: Vec<Car>
// }
//
// impl Cars {
//     fn printResult(&mut self) {
//
//     }
//     fn getWinners(&mut self) -> String {
//
//     }
// }

struct Car {
    name: String,
    position: u32
}

impl Car {
    fn getPosition(&mut self) -> String {
        "-".repeat(self.position as usize)
    }
}

fn userInput() -> String{
    let mut userInput = String::new();
    io::stdin().read_line(&mut userInput).unwrap();
    userInput
}

fn getCars() {
    let userInput = userInput();

}

fn getTryNum() -> u32 {
    let userInput = userInput();
    userInput.parse().unwrap()
}

fn main() {
    println!("경주 할 자동차 이름(이름은 쉼표(,) 기준으로 구분)");
    // let cars: Vec<Car> = getCars();

    println!("시도할 회수는 몇회인가요?");
    let tryNum = getTryNum();

    println!("실행 결과");
    println!("최종 우승자 :");
}
