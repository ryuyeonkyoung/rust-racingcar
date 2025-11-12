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
    fn new(name: String, position: u32) -> Self {
        if name.is_empty() {
            panic!("[ERROR] 차 이름은 비어있을 수 없습니다.");
        }
        Self {
            name,
            position,
        }
    }
    fn getPosition(&mut self) -> String {
        "-".repeat(self.position as usize)
    }
}

fn userInput() -> String{
    let mut userInput = String::new();
    io::stdin().read_line(&mut userInput).unwrap();
    userInput
}

fn getCars() -> Vec<Car> {
    let mut cars: Vec<Car> = Vec::new();
    let userInput = userInput();
    // 파서로 쪼개기
    let carNames: Vec<&str> = userInput.split(",").collect();
    // 차 리스트 만들기
    for name in carNames {
        let newCar: Car = Car::new(name.parse().unwrap(), 0);
        cars.push(newCar)
    }
     cars
}

fn getTryNum() -> u32 {
    let userInput = userInput();
    userInput.parse().unwrap()
}

fn main() {
    println!("경주 할 자동차 이름(이름은 쉼표(,) 기준으로 구분)");
    let cars: Vec<Car> = getCars();

    println!("시도할 회수는 몇회인가요?");
    let tryNum = getTryNum();

    println!("실행 결과");
    println!("최종 우승자 :");
}
