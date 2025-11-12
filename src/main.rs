struct Car {
    name: String,
    position: u32
}

impl Car {
    fn getPosition(&mut self) -> String {
        "-".repeat(self.position as usize)
    }
}

fn main() {
    println!("경주 할 자동차 이름(이름은 쉼표(,) 기준으로 구분)");
    println!("시도할 회수는 몇회인가요?");
    println!("실행 결과");
    println!("최종 우승자 :");
}
