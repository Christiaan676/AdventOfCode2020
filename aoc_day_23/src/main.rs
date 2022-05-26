

fn main() {
    let input = [2u8, 1, 9, 3, 4, 7, 8, 6, 5];

    let current_cup = input[0];
    let mut game = Game {current_cup, circle: input };

    for _ in 0..100 {
        game.do_move();
    }

    println!("Part1: {:?}", game.circle.iter().cycle().take_while(|&&i| i != 1).skip(1).take(8).collect::<Vec<_>>())
}

struct Game {
    circle: [u8; 9],
    current_cup: u8,
}

impl Game {
    
    fn do_move(&mut self) {
        let pick_up :Vec<_> = self.circle
            .iter()
            .cloned()
            .cycle()
            .skip_while(|&x| x == self.current_cup)
            .skip(1)
            .take(3)
            .collect();
    }
}