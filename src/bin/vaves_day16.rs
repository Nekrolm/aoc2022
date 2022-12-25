struct Graph {
    edges_to: Vec<Vec<usize>>,
}

impl Graph {
    fn shortest_distance_matrix(&self) -> aoc2022::Array2D<usize> {
        let vertex_cnt = self.edges_to.len();
        let mut distances =
            aoc2022::Array2D::from_shape_and_val((vertex_cnt, vertex_cnt), usize::MAX);

        let mut queue = std::collections::VecDeque::<usize>::new();

        for (start_v, distance_buffer) in (0..vertex_cnt).zip(distances.rows_mut()) {
            queue.push_back(start_v);
            distance_buffer[start_v] = 0;
            while let Some(cur) = queue.pop_front() {
                let d = distance_buffer[cur];
                let next = self.edges_to[cur].iter().filter_map(|&next| {
                    if distance_buffer[next] == usize::MAX {
                        distance_buffer[next] = d + 1;
                        Some(next)
                    } else {
                        None
                    }
                });
                queue.extend(next);
            }
        }
        distances
    }
}

fn parse_line(line: &str) -> (&str, i64, impl Iterator<Item = &str> + '_) {
    let (valve_from, to) = line.split_once(';').expect("; expected");
    let valve = valve_from
        .split_whitespace()
        .nth(1)
        .expect("expected valve name");
    let rate: i64 = valve_from
        .split('=')
        .next_back()
        .expect("expected rate value")
        .parse()
        .expect("should be int");
    let to = to.trim();
    let to = to
        .split_whitespace()
        .skip(4)
        .map(|s| s.trim_end_matches(','));
    (valve, rate, to)
}

fn simulate(
    cur_v: usize,
    distances: &aoc2022::Array2D<usize>,
    rates: &[i64],
    openned: &mut [bool],
    time: usize,
    current_acc: i64,
    global_max: &mut i64,
) -> i64 {
    let n = openned.len();

    let mut next_possible: Vec<_> = (0..n)
        .filter(|&next| !openned[next] && rates[next] > 0)
        .map(|to| (to, distances[(cur_v, to)] + 1))
        .filter(|(_to, cost)| *cost < time)
        .collect();

    let potential: i64 = rates
        .iter()
        .zip(&*openned)
        .filter_map(|(&val, &open)| (!open && val > 0).then_some(val))
        .sum();

    next_possible.sort_by_key(|(to, cost)| -((time - cost) as i64 * rates[*to]));

    let mut computed_max = 0;
    for (to, cost) in next_possible {
        let new_potential = potential * ((time - cost) as i64);
        if new_potential <= computed_max {
            continue;
        }
        if current_acc + new_potential <= *global_max {
            continue;
        }
        let gain = (time - cost) as i64 * rates[to];
        openned[to] = true;
        let computed_add = simulate(
            to,
            distances,
            rates,
            openned,
            time - cost,
            current_acc + gain,
            global_max,
        );
        openned[to] = false;
        *global_max = (*global_max).max(gain + computed_add + current_acc);
        computed_max = computed_max.max(gain + computed_add);
    }
    computed_max
}

fn simulate_v2(
    (cur_v1, cur_v2): (usize, usize),
    distances: &aoc2022::Array2D<usize>,
    rates: &[i64],
    openned: &mut [bool],
    non_zero: &[usize],
    (time_1, time_2): (usize, usize),
    current_acc: i64,
    global_max: &mut i64,
    rates_potential: i64,
    discarded: &mut usize,
    computed: &mut usize,
) -> i64 {

    let next_possible_v1 = non_zero.iter().copied().filter(|&next| !openned[next]);

    let next_possible_v2 = next_possible_v1.clone();

    let next_possible_v1 = next_possible_v1
        .map(|to| (to, distances[(cur_v1, to)] + 1))
        .filter(|(_to, cost)| *cost < time_1);
    let next_possible_v2 = next_possible_v2
        .map(|to| (to, distances[(cur_v2, to)] + 1))
        .filter(|(_to, cost)| *cost < time_2);

    let mut next_possible: Vec<_> = next_possible_v1
        .flat_map(move |p1| {
            std::iter::once((Some(p1), None))
                .chain(
                    next_possible_v2
                        .clone()
                        .filter(move |&(to, _)| to != p1.0)
                        .map(move |p2| (Some(p1), Some(p2))),
                )
                .chain(next_possible_v2.clone().map(|p2| (None, Some(p2))))
        })
        .collect();

    let compute_gain = |next: Option<(usize, usize)>, time: usize| match next {
        Some((to, cost)) => (time - cost) as i64 * rates[to],
        None => 0,
    };

    let compute_potential =
        |next1: Option<(usize, usize)>, next2: Option<(usize, usize)>, visited: &[bool]| -> i64 {
            non_zero
                .iter()
                .copied()
                .filter(|&next| !visited[next])
                .map(move |to| {
                    let one = next1.map_or(0, |(from, cost)| {
                        let have_time = time_1 - cost;
                        let next_cost = distances[(from, to)] + 1;
                        if next_cost < have_time {
                            rates[to] * (have_time - next_cost) as i64
                        } else {
                            0
                        }
                    });
                    let two = next2.map_or(0, |(from, cost)| {
                        let have_time = time_2 - cost;
                        let next_cost = distances[(from, to)] + 1;
                        if next_cost < have_time {
                            rates[to] * (have_time - next_cost) as i64
                        } else {
                            0
                        }
                    });
                    one.max(two)
                })
                .sum()
        };

    let compute_approx_potential =
        move |next1: Option<(usize, usize)>, next2: Option<(usize, usize)>| match (next1, next2) {
            (Some((to1, cost1)), Some((to2, cost2))) => {
                (rates_potential - rates[to1] - rates[to2])
                    * (time_1 - cost1).max(time_2 - cost2).saturating_sub(1) as i64
            }
            (Some((to1, cost1)), None) => (rates_potential - rates[to1]) * (time_1 - cost1).saturating_sub(1) as i64,
            (None, Some((to2, cost2))) => (rates_potential - rates[to2]) * (time_2 - cost2).saturating_sub(1) as i64,
            _ => 0,
        };

    next_possible.sort_by_key(|(p1, p2)| {
        -compute_gain(*p1, time_1) - compute_gain(*p2, time_2)
    });

    let visit = |p: Option<(usize, usize)>, visited: &mut [bool], status: bool| match p {
        Some((to, _)) => visited[to] = status,
        _ => {}
    };

    let mut computed_max = 0;
    for (next1, next2) in next_possible {
        let gain1 = compute_gain(next1, time_1);
        let gain2 = compute_gain(next2, time_2);

        let approx_potential = compute_approx_potential(next1, next2);

        if gain1 + gain2 + approx_potential <= computed_max {
            *discarded += 1;
            continue;
        }

        if gain1 + gain2 + approx_potential + current_acc <= *global_max {
            *discarded += 1;
            continue;
        }

        visit(next1, openned, true);
        visit(next2, openned, true);

        let new_potential = compute_potential(next1, next2, openned);

        visit(next1, openned, false);
        visit(next2, openned, false);

        if gain1 + gain1 + new_potential <= computed_max {
            *discarded += 1;
            continue;
        }

        if current_acc + gain1 + gain2 + new_potential <= *global_max {
            *discarded += 1;
            continue;
        }

        visit(next1, openned, true);
        visit(next2, openned, true);

        let computed_add = simulate_v2(
            (next1.map_or(cur_v1, |p| p.0), next2.map_or(cur_v2, |p| p.0)),
            distances,
            rates,
            openned,
            &non_zero,
            (
                time_1 - next1.map_or(0, |p| p.1),
                time_2 - next2.map_or(0, |p| p.1),
            ),
            current_acc + gain1 + gain2,
            global_max,
            rates_potential - next1.map_or(0, |p| rates[p.0]) - next2.map_or(0, |p| rates[p.0]),
            discarded,
            computed,
        );

        visit(next1, openned, false);
        visit(next2, openned, false);

        *global_max = (*global_max).max(gain1 + gain2 + computed_add + current_acc);
        computed_max = computed_max.max(gain1 + gain2 + computed_add);
    }

    *computed += 1;

    computed_max
}

fn main() {
    let mut mapper = std::collections::HashMap::<String, usize>::new();
    let mut vertex_mapper = move |v: &str| -> usize {
        let cur_cnt = mapper.len();
        *mapper.entry(v.to_string()).or_insert(cur_cnt)
    };

    let mut graph = Graph {
        edges_to: Default::default(),
    };

    let mut rates = Vec::<i64>::new();

    aoc2022::parse_line_by_line(aoc2022::get_input_file(), |line| {
        let (v, rate, next) = parse_line(line);
        let v = (vertex_mapper)(v);
        let next: Vec<_> = next.map(&mut vertex_mapper).collect();
        (v, rate, next)
    })
    .for_each(|(v, rate, next)| {
        if v >= graph.edges_to.len() {
            graph.edges_to.resize_with(v + 1, Default::default);
            rates.resize_with(v + 1, Default::default);
        }

        rates[v] = rate;
        graph.edges_to[v] = next
    });

    let non_zero_positions: Vec<usize> = rates
        .iter()
        .enumerate()
        .filter_map(|(idx, &val)| (val > 0).then_some(idx))
        .collect();

    let start = (vertex_mapper)("AA");
    let vertices = rates.len();
    let shortest_distances = graph.shortest_distance_matrix();

    let mut openned = vec![false; vertices];

    let mut global_max = 0;
    // let answer = simulate(start, &shortest_distances, &rates, &mut openned, 30, 0, &mut global_max);
    let mut discarded = 0;
    let mut computed = 0;
    let answer = simulate_v2(
        (start, start),
        &shortest_distances,
        &rates,
        &mut openned,
        &non_zero_positions,
        (26, 26),
        0,
        &mut global_max,
        rates.iter().sum(),
        &mut discarded,
        &mut computed
    );

    println!("{answer} {discarded} {computed}");
}
