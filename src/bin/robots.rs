fn main() {
    // // p1
    let score: usize = aoc2022::parse_line_by_line(aoc2022::get_input_file(), parse_blueprint)
        .map(|bprint| bprint.estimate(State::start_p1()))
        .zip(1..)
        .map(|(x, y)| x * y)
        .sum();
    println!("{score}")
    // p2
    // let score: usize = aoc2022::parse_line_by_line(aoc2022::get_input_file(), parse_blueprint)
    //     .take(3)
    //     .map(|bprint| bprint.estimate(State::start_p2()))
    //     .inspect(|d| {
    //         dbg!(d);
    //     })
    //     .product();

    // println!("{score}")
}

fn parse_blueprint(line: &str) -> Blueprint {
    let (_, bp) = line.split_once(':').expect("wrong format");
    let bp = bp.trim();
    let mut robots = bp.split('.').map(str::trim);
    let ore = robots.next().expect("wrong format");
    let clay = robots.next().expect("wrong format");
    let obsidian = robots.next().expect("wrong format");
    let geod = robots.next().expect("wrong format");

    Blueprint {
        obsidian: ObsidianRobot::parse_line(obsidian),
        geode: GeodeRobot::parse_line(geod),
        clay: ClayRobot::parse_line(clay),
        ore: OreRobot::parse_line(ore),
    }
}

struct OreRobot {
    need_ore: usize,
}

struct ClayRobot {
    need_ore: usize,
}

struct ObsidianRobot {
    need_ore: usize,
    need_clay: usize,
}

struct GeodeRobot {
    need_ore: usize,
    need_obsidian: usize,
}

impl OreRobot {
    fn parse_line(line: &str) -> Self {
        let ore = line.split_whitespace().nth_back(1).expect("wrong format");
        Self {
            need_ore: ore.parse().expect("expected int"),
        }
    }
}

impl ClayRobot {
    fn parse_line(line: &str) -> Self {
        let ore = line.split_whitespace().nth_back(1).expect("wrong format");
        Self {
            need_ore: ore.parse().expect("expected int"),
        }
    }
}

impl ObsidianRobot {
    fn parse_line(line: &str) -> Self {
        let mut tokens = line.split_whitespace();
        let clay = tokens.nth_back(1).expect("wrong format");
        let ore = tokens.nth_back(2).expect("wrong format");
        Self {
            need_ore: ore.parse().expect("expected int"),
            need_clay: clay.parse().expect("expected int"),
        }
    }
}

impl GeodeRobot {
    fn parse_line(line: &str) -> Self {
        let mut tokens = line.split_whitespace();
        let obsidian = tokens.nth_back(1).expect("wrong format");
        let ore = tokens.nth_back(2).expect("wrong format");
        Self {
            need_ore: ore.parse().expect("expected int"),
            need_obsidian: obsidian.parse().expect("expected int"),
        }
    }
}

struct Blueprint {
    ore: OreRobot,
    clay: ClayRobot,
    obsidian: ObsidianRobot,
    geode: GeodeRobot,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    time: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

#[derive(Debug, Clone, Copy)]
struct NewResources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

trait StateUpdater {
    fn try_update(&self, state: State) -> Option<State>;

    fn enough(&self, state: State, bp: &Blueprint) -> bool;
}

impl StateUpdater for OreRobot {
    fn try_update(&self, state: State) -> Option<State> {
        (state.ore >= self.need_ore).then_some(State {
            ore: state.ore - self.need_ore,
            ore_robots: state.ore_robots + 1,
            ..state
        })
    }

    fn enough(&self, state: State, bp: &Blueprint) -> bool {
        state.ore_robots
            >= bp
                .ore
                .need_ore
                .max(bp.clay.need_ore)
                .max(bp.obsidian.need_ore)
                .max(bp.geode.need_ore)
    }
}

impl StateUpdater for ClayRobot {
    fn try_update(&self, state: State) -> Option<State> {
        (state.ore >= self.need_ore).then_some(State {
            ore: state.ore - self.need_ore,
            clay_robots: state.clay_robots + 1,
            ..state
        })
    }

    fn enough(&self, state: State, bp: &Blueprint) -> bool {
        state.clay_robots >= bp.obsidian.need_clay
    }
}

impl StateUpdater for ObsidianRobot {
    fn try_update(&self, state: State) -> Option<State> {
        (state.ore >= self.need_ore && state.clay >= self.need_clay).then_some(State {
            ore: state.ore - self.need_ore,
            clay: state.clay - self.need_clay,
            obsidian_robots: state.obsidian_robots + 1,
            ..state
        })
    }

    fn enough(&self, state: State, bp: &Blueprint) -> bool {
        state.obsidian_robots >= bp.geode.need_obsidian
    }
}

impl StateUpdater for GeodeRobot {
    fn try_update(&self, state: State) -> Option<State> {
        (state.ore >= self.need_ore && state.obsidian >= self.need_obsidian).then_some(State {
            ore: state.ore - self.need_ore,
            obsidian: state.obsidian - self.need_obsidian,
            geode_robots: state.geode_robots + 1,
            ..state
        })
    }

    fn enough(&self, state: State, bp: &Blueprint) -> bool {
        false
    }
}

struct Noop;

impl StateUpdater for Noop {
    fn try_update(&self, state: State) -> Option<State> {
        Some(state)
    }

    fn enough(&self, state: State, bp: &Blueprint) -> bool {
        false
    }
}

impl State {
    fn start_p1() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            time: 24,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn start_p2() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            time: 32,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn apply(self, r: NewResources) -> Self {
        Self {
            ore: self.ore + r.ore,
            clay: self.clay + r.clay,
            obsidian: self.obsidian + r.obsidian,
            ..self
        }
    }

    fn produce_new_resources(&self) -> NewResources {
        NewResources {
            ore: self.ore_robots,
            clay: self.clay_robots,
            obsidian: self.obsidian_robots,
            geode: self.geode_robots,
        }
    }

    fn tick(self) -> Self {
        Self {
            time: self.time - 1,
            ..self
        }
    }

    fn only_noop_can_be(self, bp: &Blueprint) -> bool {
        let updaters: [&dyn StateUpdater; 5] = [&bp.ore, &bp.clay, &bp.obsidian, &bp.geode, &Noop];
        updaters
            .into_iter()
            .filter(move |upd| !upd.enough(self, &bp))
            .flat_map(move |upd| upd.try_update(self))
            .count()
            == 1
    }

    fn can_build_something(self, bp: &Blueprint) -> bool {
        let updaters: [&dyn StateUpdater; 4] = [&bp.ore, &bp.clay, &bp.obsidian, &bp.geode];
        updaters
            .into_iter()
            .find_map(|upd| upd.try_update(self))
            .is_some()
    }

    fn build_robots_and_produce_resources<'b>(
        &self,
        bp: &'b Blueprint,
    ) -> impl Iterator<Item = (Self, usize)> + 'b {
        let produced = self.produce_new_resources();
        let mut gain = produced.geode;
        let mut state = *self;
        while (!state.can_build_something(bp) || state.only_noop_can_be(bp)) && state.time > 0 {
            state = state.tick();
            state = state.apply(produced);
            gain = gain + produced.geode;
        }

        if state.time == 0 {
            aoc2022::either::Either::Left(std::iter::once((state, gain)))
        } else {
            let with_build = if let Some(next) = bp.geode.try_update(state) {
                aoc2022::either::Either::Left(std::iter::once((next.tick().apply(produced), gain)))
            } else {
                let updaters: [&dyn StateUpdater; 4] = [&bp.obsidian, &bp.clay, &bp.ore, &Noop];
                aoc2022::either::Either::Right(
                    updaters
                        .into_iter()
                        .filter(move |upd| !upd.enough(state, &bp))
                        .flat_map(move |upd| upd.try_update(state))
                        .map(move |state| state.apply(produced))
                        .map(State::tick)
                        .map(move |s| (s, gain)),
                )
            };

            aoc2022::either::Either::Right(with_build)
        }
    }

    fn approx_max_geode_gain(&self) -> usize {
        let ticks = self.time;
        let max = self.geode_robots + ticks - 1;
        (self.geode_robots + max) * ticks / 2
    }
}

impl Blueprint {
    fn estimate(&self, start: State) -> usize {
        let mut cached_values = std::collections::HashMap::<State, usize>::new();
        let mut global_max = 0;
        self.estimate_impl(start, 0, &mut global_max, &mut cached_values)
    }

    fn estimate_impl(
        &self,
        state: State,
        accumulated_geods: usize,
        global_max: &mut usize,
        cached_values: &mut std::collections::HashMap<State, usize>,
    ) -> usize {
        if accumulated_geods + state.approx_max_geode_gain() <= *global_max {
            return *global_max;
        }

        if let Some(val) = cached_values.get(&state) {
            return *val;
        }


        if state.time == 0 {
            cached_values.insert(state, accumulated_geods);
            *global_max = (*global_max).max(accumulated_geods);
            return accumulated_geods;
        }

        let max_val = state
            .build_robots_and_produce_resources(self)
            .map(|(new_state, gain)| {
                *global_max = (*global_max).max(accumulated_geods + gain);
                let val = self.estimate_impl(
                    new_state,
                    accumulated_geods + gain,
                    global_max,
                    cached_values,
                );
                *global_max = (*global_max).max(val);
                val
            })
            .max()
            .expect("should be at least one element");

        cached_values.insert(state, max_val);
        max_val
    }
}
