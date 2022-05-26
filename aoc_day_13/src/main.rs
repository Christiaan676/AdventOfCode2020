
use std::collections::HashSet;

fn main() {
    let time = 1001612;
    let bus_nr = [19,41,37,821,13,17,29,463,23];

    let mut wait_times: Vec<(usize, usize)> = bus_nr.iter()
        .map(|nr| (*nr, wait_time(time, *nr)))
        .collect();
    wait_times.sort_by(|(_, a_wait), (_, b_wait)| a_wait.cmp(b_wait));

    let (bus, wait_min) = wait_times[0];
    println!("Part1: {}", bus * wait_min);


    let input = "19,x,x,x,x,x,x,x,x,41,x,x,x,37,x,x,x,x,x,821,x,x,x,x,x,x,x,x,x,x,x,x,13,x,x,x,17,x,x,x,x,x,x,x,x,x,x,x,29,x,463,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";
    let mut bus: HashSet<(u32, usize)> = input.split(',')
        .enumerate()
        .filter_map(|(offset, bus)| bus.parse::<u32>().ok().map(|bus| (bus, offset)))
        .collect();

    // All bus numbers are prime numbers. This means that once a time t is found where bus A and B are pressent
    // on there respectife offsets. Than that will also be the case on t + n*A*B
    // For more info: Chinese remainder theorem
    let mut time_step: usize = 1;
    let mut time = 0;
    while !bus.is_empty() {
        // Check if currect pos matches
        let (matched_bus, other_bus) = bus.iter()
            .partition::<HashSet<_>, _>(|(bus, offset)| is_pressent(time + offset, *bus));
        bus = other_bus;

        if !matched_bus.is_empty() {
            println!("time: {} {:?}", time, matched_bus);
        }

        time_step *= matched_bus.iter().map(|(bus, _)| *bus as usize).product::<usize>();
        time += time_step;
    }
    time -= time_step; //Correct for the last increment

    println!("Part 2: {}", time);
}

fn is_pressent(time: usize, bus :u32) -> bool {
    time % (bus as usize) == 0
}

fn wait_time(time: usize, bus_nr: usize) -> usize {
    (time / bus_nr + 1) * bus_nr - time
}
