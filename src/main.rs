use std::hash::RandomState;
use std::io;
use std::num::FpCategory::Infinite;
use rand::{random, random_range};

struct Cars {
    cars: Vec<Car>
}

impl Cars {
    fn new (cars: Vec<Car>) -> Self{
        Self {
            cars,
        }
    }
    fn get_max_position(&mut self) -> u32 {
        let mut max = 0;
        for car in &self.cars {
            if car.position > max {
                max = car.position;
            }
        }
        max
    }
    fn get_winners_name(&mut self) -> String {
        let mut winners_name = String::new();
        let max = self.get_max_position();
        // for문을 돌면서 유저의 이름 추가 (a, b)
        for car in &self.cars { // TODO: 소유권 문제 해결
            if max == car.position {
                winners_name.push_str(&car.name);
            }
        }
        winners_name
    }
}

struct Car {
    name: String,
    position: u32
}

impl Car {
    fn new(name: String, position: u32) -> Self {
        if name.is_empty() {
            panic!("[ERROR] 차 이름은 비어있을 수 없습니다."); // TODO: Result로 복구 가능한 에러 처리하기
        }
        Self {
            name,
            position,
        }
    }
    fn one_step(&mut self) {
        self.position = self.position + 1;
    }
    fn get_position_string(&mut self) -> String {
        "-".repeat(self.position as usize)
    }
    fn get_result(&mut self) -> String {
        let mut result = self.name.clone(); // TODO: “If you see clone(), ask yourself why.”
        result.push_str(" : ");
        result.push_str(&self.get_position_string());
        result
    }
}

// ------ 입력 ------
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

fn user_input() -> String{
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

fn get_try_num() -> u32 {
    let raw_input = user_input();
    let try_num = raw_input.parse::<u32>().unwrap();
    try_num
}

// ----- 레이싱 시작 ------
fn start_racing(try_num: u32, cars: Cars) -> Vec<Cars> {
    todo!()
}

fn can_move() -> bool {
    let random_num = random_range(0..=9);
    if random_num >= 4 {
        return true;
    }
    false
}

fn main() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
    let cars: Cars = Cars::new(get_cars());

    println!("시도할 회수는 몇회인가요?");
    let try_num = get_try_num();

    println!("실행 결과");
    let history: Vec<Cars> = start_racing(try_num, cars); // TODO: cars move 문제 해결

    println!("최종 우승자 :");
    // print_results(cars.get_winners_name());
}
