use itertools::Itertools;
use rand::random_range;
use std::io;

#[derive(Clone)]
struct Cars {
    cars: Vec<Car>
}

impl Cars {
    fn new (cars: Vec<Car>) -> Self{
        Self {
            cars,
        }
    }
    fn one_turn(&mut self) {
        for car in &mut self.cars {
            car.one_turn();
        }
    }
    fn get_max_position(&self) -> u32 {
        let mut max = 0;
        for car in &self.cars {
            if car.position > max {
                max = car.position;
            }
        }
        max
    }
    fn get_winners_name(&self) -> String {
        let max = self.get_max_position();
        self.cars
            .iter()
            .filter(|c| c.position == max)
            .map(|c| c.name.as_str())
            .join(", ")
    }
    fn get_result(&self) -> String {
        let mut result = String::new();
        for car in &self.cars {
            result.push_str(&car.get_result());
            result.push('\n');
        }
        result
    }
}

#[derive(Clone)]
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
    fn one_turn(&mut self) {
        if (Self::can_move()) {
            self.position += 1;
        }
    }
    fn can_move() -> bool {
        let random_num = random_range(0..=9);
        random_num >= 4
    }
    fn get_position_string(&self) -> String {
        "-".repeat(self.position as usize)
    }
    fn get_result(&self) -> String {
        format!("{} : {}", self.name, self.get_position_string())
    }
}

// ------ 입력 ------
fn get_cars() -> Vec<Car> {
    let mut cars: Vec<Car> = Vec::new();
    let user_input = user_input();
    let car_names: Vec<&str> = user_input.split(",").collect();
    for name in car_names {
        let new_car: Car = Car::new(name.trim().to_string(), 0);
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
    // TODO: loop를 통해 올바를 정수를 입력받을 때까지 시도
    let raw_input = user_input();
    let try_num = raw_input.parse::<u32>().unwrap();
    try_num
}

// ------ 출력 ------
fn get_history(history: &Vec<Cars>) -> String {
    let mut history_display = String::new();
    for cars in history {
        history_display.push_str(&cars.get_result());
        history_display.push('\n');
    }
    history_display
}

// ----- 레이싱 시작 ------
fn start_racing(try_num: u32, mut cars: Cars) -> Vec<Cars> {
    let mut history: Vec<Cars> = Vec::new();
    for _ in 1..=try_num {
        cars.one_turn();
        history.push(cars.clone()); // TODO: 대체 방법 고민
    }
    history
}

fn main() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
    let cars: Cars = Cars::new(get_cars());

    println!("시도할 회수는 몇회인가요?");
    let try_num = get_try_num();

    println!("실행 결과");
    let history: Vec<Cars> = start_racing(try_num, cars);
    println!("{}", get_history(&history));

    println!("최종 우승자 :");
    let result: String = history.last().unwrap().get_winners_name();
    println!("{}", result);
}
