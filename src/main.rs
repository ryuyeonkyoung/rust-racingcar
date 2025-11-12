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
    fn get_position(&mut self) -> String {
        "-".repeat(self.position as usize)
    }
    fn get_result(&mut self) -> String {
        let mut result = self.name;
        result.push_str(" : ");
        result.push_str(self.get_position());
        result
    }
}

fn user_input() -> String{
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

fn get_cars() -> Vec<Car> {
    let mut cars: Vec<Car> = Vec::new();
    let user_input = user_input();
    // 파서로 쪼개기
    let car_names: Vec<&str> = user_input.split(",").collect();
    // 차 리스트 만들기
    for name in car_names {
        let new_car: Car = Car::new(name.parse().unwrap(), 0);
        cars.push(new_car)
    }
     cars
}

fn get_try_num() -> u32 {
    let raw_input = user_input();
    let try_num = raw_input.parse::<u32>().unwrap();
    try_num
}

fn main() {
    println!("경주 할 자동차 이름(이름은 쉼표(,) 기준으로 구분)");
    let cars: Vec<Car> = get_cars();

    println!("시도할 회수는 몇회인가요?");
    let try_num = get_try_num();

    println!("실행 결과");
    // 실행 후 Cars 리스트 반환
    // 반환값을 파라미터로 넣어 실행 결과 출력

    println!("최종 우승자 :");
}
