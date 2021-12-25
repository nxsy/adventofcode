// cargo run --example day23 -- (part1|part2) (example_input|final_input)

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day23", about = "Advent of Code 2021 Day 23")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Position {
    Hallway(usize),
    Room(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Actor {
    target_room: usize,
    position: Position,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State<const A: usize, const R: usize> {
    actors: [Actor; A],
    rooms: [Room<R>; 4],
    hallway: [Option<(Actor, usize)>; 11],
}

#[derive(PartialEq, Eq)]
struct Cost<const A: usize, const R: usize>(usize, State<A, R>);

impl<const A: usize, const R: usize> PartialOrd for Cost<A, R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0).map(|o| o.reverse())
    }
}

impl<const A: usize, const R: usize> Ord for Cost<A, R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Room<const R: usize> {
    room_number: usize,
    occupants: [Option<(Actor, usize)>; R],
    last_filled_slot: Option<usize>,
}

impl<const R: usize> Room<R> {
    fn empty(room_number: usize) -> Self {
        Self {
            room_number,
            occupants: [None; R],
            last_filled_slot: None,
        }
    }

    fn can_be_moved_to(&self) -> bool {
        // Can be moved to only if any occupants are occupants that belong in that room
        let non_matching_occupants = self.occupants.iter().filter(|o| {
            if let Some((actor, _)) = o {
                actor.target_room != self.room_number
            } else {
                false
            }
        });
        non_matching_occupants.count() == 0
    }

    fn can_be_moved_from(&self) -> bool {
        // Can be moved from if any occupants are occupants that don't belong in that room
        let non_matching_occupants = self.occupants.iter().filter(|o| {
            if let Some((actor, _)) = o {
                actor.target_room != self.room_number
            } else {
                false
            }
        });
        non_matching_occupants.count() != 0
    }

    fn with_actors(room_number: usize, actors: &[Actor]) -> Self {
        let mut r = Room::empty(room_number);
        for (i, actor) in actors.iter().enumerate() {
            if let Position::Room(actor_room_number, position) = actor.position {
                if actor_room_number == room_number {
                    r.occupants[position] = Some((*actor, i));
                    r.last_filled_slot = Some(if let Some(last_filled_slot) = r.last_filled_slot {
                        last_filled_slot.max(position)
                    } else {
                        position
                    });
                }
            }
        }
        r
    }
}

fn hallway_with_actors(actors: &[Actor]) -> [Option<(Actor, usize)>; 11] {
    let mut hallway = [None; 11];
    for (i, actor) in actors.iter().enumerate() {
        if let Position::Hallway(p) = actor.position {
            hallway[p] = Some((*actor, i));
        }
    }
    hallway
}

fn hallway_options(
    position: usize,
    include_room_entrances: bool,
    hallway: &[Option<(Actor, usize)>; 11],
) -> HashSet<usize> {
    let non_stopping_hallway_positions = HashSet::from([2, 4, 6, 8]);
    let mut options = HashSet::new();

    if position != 0 {
        let mut test_position = position - 1;
        loop {
            if hallway[test_position].is_some() {
                break;
            }
            if include_room_entrances || !non_stopping_hallway_positions.contains(&test_position) {
                options.insert(test_position);
            }
            if test_position == 0 {
                break;
            }
            test_position -= 1;
        }
    }

    if position != 10 {
        let mut test_position = position + 1;
        loop {
            if hallway[test_position].is_some() {
                break;
            }
            if include_room_entrances || !non_stopping_hallway_positions.contains(&test_position) {
                options.insert(test_position);
            }
            if test_position == 10 {
                break;
            }
            test_position += 1;
        }
    }

    options
}

impl<const A: usize, const R: usize> State<A, R> {
    fn new(actors: [Actor; A]) -> Self {
        let rooms = [
            Room::with_actors(0, &actors),
            Room::with_actors(1, &actors),
            Room::with_actors(2, &actors),
            Room::with_actors(3, &actors),
        ];
        let hallway = hallway_with_actors(&actors);

        Self {
            actors,
            rooms,
            hallway,
        }
    }

    fn done(&self) -> bool {
        assert!(self.valid());
        for a in self.actors {
            if let Position::Room(room_number, _) = a.position {
                if a.target_room != room_number {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn valid(&self) -> bool {
        let mut positions = HashSet::new();
        for a in self.actors {
            if positions.contains(&a.position) {
                return false;
            }
            positions.insert(a.position);
        }
        true
    }

    fn display(&self) -> String {
        // Both for human use and to use as serialized format...
        let mut s = String::new();
        let actor_display_lookup = HashMap::from([(0, "A"), (1, "B"), (2, "C"), (3, "D")]);
        s.push_str("#############\n");
        s.push('#');
        for actor in self.hallway {
            s.push_str(if let Some((actor, _)) = actor {
                actor_display_lookup[&actor.target_room]
            } else {
                "."
            });
        }
        s.push_str("#\n");
        for room_p in 0..R {
            s.push_str(if room_p == 0 { "###" } else { "  #" });
            for room_number in 0..4 {
                let o = self.rooms[room_number].occupants[R - room_p - 1];
                if let Some((a, _)) = o {
                    s.push_str(actor_display_lookup[&a.target_room]);
                } else {
                    s.push('.');
                }
                s.push('#')
            }
            s.push_str(if room_p == 0 { "##" } else { "  " });
            s.push('\n');
        }

        s.push_str("  #########\n");

        s
    }

    fn new_state_with_actor_at_new_position(
        &self,
        actor_id: usize,
        position: Position,
    ) -> State<A, R> {
        let mut new_actors = self.actors;
        let actor = self.actors[actor_id];
        new_actors[actor_id] = Actor {
            target_room: actor.target_room,
            position,
        };
        State::new(new_actors)
    }

    fn valid_moves(&self) -> Vec<(State<A, R>, usize)> {
        let mut moves = Vec::new();

        for (position, o) in self.hallway.iter().enumerate() {
            if let Some((actor, actor_id)) = o {
                self.add_valid_hallway_moves(actor, position, actor_id, &mut moves);
            }
        }

        for r in &self.rooms {
            if !r.can_be_moved_from() {
                continue;
            }
            if let Some(last_slot) = r.last_filled_slot {
                assert!(r.occupants[last_slot].is_some());
                if let Some((actor, actor_id)) = r.occupants[last_slot] {
                    let cost_per_step = 10_usize.pow(actor.target_room as u32);
                    let into_hallway_steps = R - last_slot;
                    let current_room_hallway_position = 2 + 2 * r.room_number;

                    let reachable =
                        hallway_options(current_room_hallway_position, false, &self.hallway);
                    for p in reachable.iter() {
                        let hallway_steps =
                            (*p as i32 - current_room_hallway_position as i32).abs() as usize;
                        let total_steps = hallway_steps + into_hallway_steps;
                        let cost = total_steps * cost_per_step;
                        let position = Position::Hallway(*p);
                        let new_state =
                            self.new_state_with_actor_at_new_position(actor_id, position);
                        moves.push((new_state, cost));
                    }
                }
            }
        }

        moves
    }

    fn add_valid_hallway_moves(
        &self,
        actor: &Actor,
        position: usize,
        actor_id: &usize,
        moves: &mut Vec<(State<A, R>, usize)>,
    ) {
        let cost_per_step = 10_usize.pow(actor.target_room as u32);
        let room_hallway_target = 2 + 2 * actor.target_room;
        let reachable = hallway_options(position, true, &self.hallway);
        let target_room = &self.rooms[actor.target_room];
        if reachable.contains(&room_hallway_target) && target_room.can_be_moved_to() {
            let new_room_position = match target_room.last_filled_slot {
                Some(p) => p + 1,
                None => 0,
            };
            let new_state = self.new_state_with_actor_at_new_position(
                *actor_id,
                Position::Room(actor.target_room, new_room_position),
            );
            let hallway_steps = (position as i32 - room_hallway_target as i32).abs() as usize;
            let full_steps = hallway_steps + R - new_room_position;
            moves.push((new_state, full_steps * cost_per_step));
        }
    }
}

fn solve(input: Input) -> Result<()> {
    let room_stacks = match input {
        Input::ExampleInput => [[0, 1], [3, 2], [2, 1], [0, 3]],
        Input::FinalInput => [[1, 0], [2, 3], [3, 1], [0, 2]],
    };

    let mut actors = Vec::new();
    for (i, &room) in room_stacks.iter().enumerate() {
        for (j, &a) in room.iter().enumerate() {
            actors.push(Actor {
                target_room: a,
                position: Position::Room(i, j),
            })
        }
    }
    let actors = [
        actors[0], actors[1], actors[2], actors[3], actors[4], actors[5], actors[6], actors[7],
    ];
    let starting_state = State::<8, 2>::new(actors);

    let (final_cost, explored) = calculate_cost(starting_state);

    println!("Final cost: {}, explored {}", final_cost, explored);

    Ok(())
}

fn part2(input: Input) -> Result<()> {
    let room_stacks = match input {
        Input::ExampleInput => [[0, 3, 3, 1], [3, 1, 2, 2], [2, 0, 1, 1], [0, 2, 0, 3]],
        Input::FinalInput => [[1, 3, 3, 0], [2, 1, 2, 3], [3, 0, 1, 1], [0, 2, 0, 2]],
    };

    let mut actors = Vec::new();
    for (i, &room) in room_stacks.iter().enumerate() {
        for (j, &a) in room.iter().enumerate() {
            actors.push(Actor {
                target_room: a,
                position: Position::Room(i, j),
            })
        }
    }
    let actors: [Actor; 16] = [
        // Really?  No pre-existing .into()?
        actors[0], actors[1], actors[2], actors[3], actors[4], actors[5], actors[6], actors[7],
        actors[8], actors[9], actors[10], actors[11], actors[12], actors[13], actors[14],
        actors[15],
    ];
    let starting_state = State::<16, 4>::new(actors);

    let (final_cost, explored) = calculate_cost(starting_state);

    println!("Final cost: {}, explored {}", final_cost, explored);

    Ok(())
}

fn calculate_cost<const A: usize, const R: usize>(starting_state: State<A, R>) -> (usize, usize) {
    let mut stack = BinaryHeap::new();
    stack.push(Cost(0, starting_state));
    let mut costs = HashMap::new();
    let mut final_cost = usize::MAX;
    while let Some(Cost(cost, state)) = stack.pop() {
        assert!(state.valid());
        let serialized = state.display();
        if let Some(current_cost) = costs.get(&serialized) {
            if cost >= *current_cost {
                continue;
            }
        }
        costs.insert(serialized, cost);

        if state.done() {
            final_cost = final_cost.min(cost);
            continue;
        }

        let moves = state.valid_moves();
        for (new_state, move_cost) in moves {
            stack.push(Cost(cost + move_cost, new_state));
        }
    }
    (final_cost, costs.len())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    match args.part {
        Part::Part1 => solve(args.input),
        Part::Part2 => part2(args.input),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        calculate_cost, hallway_options, hallway_with_actors, Actor, Position, Room, State,
    };

    #[test]
    fn test_can_be_moved_to() {
        let mut room = Room::<2>::empty(0);
        assert!(room.can_be_moved_to());

        let non_match_actor = Actor {
            target_room: 1,
            position: Position::Room(0, 0),
        };

        room.occupants[0] = Some((non_match_actor, 0));

        assert!(!room.can_be_moved_to());

        let match_actor = Actor {
            target_room: 0,
            position: Position::Room(0, 0),
        };

        room.occupants[0] = Some((match_actor, 0));
        assert!(room.can_be_moved_to());
    }

    #[test]
    fn test_with_actors() {
        let non_match_actor = Actor {
            target_room: 1,
            position: Position::Room(1, 1),
        };

        let match_actor = Actor {
            target_room: 0,
            position: Position::Room(1, 0),
        };
        let actors = &[non_match_actor, match_actor];
        let room = Room::<2>::with_actors(1, actors);

        println!("{:?}", room);
        assert!(room.last_filled_slot == Some(1));
        assert!(room.occupants[0] == Some((match_actor, 1)));
        assert!(room.occupants[1] == Some((non_match_actor, 0)));

        let room = Room::<2>::with_actors(0, actors);
        assert!(room.last_filled_slot == None);
        assert!(room.occupants[0] == None);
        assert!(room.occupants[1] == None);
    }

    #[test]
    fn test_hallway_with_actors() {
        let actors = [
            Actor {
                target_room: 1,
                position: Position::Room(1, 1),
            },
            Actor {
                target_room: 1,
                position: Position::Hallway(10),
            },
        ];

        let hallway = hallway_with_actors(&actors);

        for i in 0..11 {
            let expected = if i == 10 { Some((actors[1], 1)) } else { None };
            assert_eq!(hallway[i], expected);
        }

        let actors = [
            Actor {
                target_room: 1,
                position: Position::Hallway(4),
            },
            Actor {
                target_room: 1,
                position: Position::Hallway(10),
            },
        ];

        let hallway = hallway_with_actors(&actors);

        for i in 0..11 {
            let expected = if i == 10 {
                Some((actors[1], 1))
            } else if i == 4 {
                Some((actors[0], 0))
            } else {
                None
            };
            assert_eq!(hallway[i], expected);
        }
    }

    #[test]
    fn test_hallway_options() {
        let actors = [
            Actor {
                target_room: 1,
                position: Position::Hallway(3),
            },
            Actor {
                target_room: 1,
                position: Position::Hallway(9),
            },
        ];

        let hallway = hallway_with_actors(&actors);

        let expected_wo_entrances = HashSet::from([5, 7]);
        let expected_w_entrances = HashSet::from([5, 6, 7, 8]);
        let options_wo_entrances = hallway_options(4, false, &hallway);
        assert_eq!(options_wo_entrances, expected_wo_entrances);
        let options_w_entrances = hallway_options(4, true, &hallway);
        assert_eq!(options_w_entrances, expected_w_entrances);
    }

    #[test]
    fn empty_hallway_options() {
        let actors = [];

        let hallway = hallway_with_actors(&actors);

        let expected_wo_entrances = HashSet::from([0, 1, 3, 5, 7, 9, 10]);
        let expected_w_entrances = HashSet::from_iter((0..4_usize).chain(5..11));
        let options_wo_entrances = hallway_options(4, false, &hallway);
        assert_eq!(options_wo_entrances, expected_wo_entrances);
        let options_w_entrances = hallway_options(4, true, &hallway);
        assert_eq!(options_w_entrances, expected_w_entrances);
    }

    #[test]
    fn test_valid_moves() {
        let actors = [
            Actor {
                target_room: 1,
                position: Position::Room(2, 1),
            },
            Actor {
                target_room: 1,
                position: Position::Room(1, 0),
            },
        ];

        let state = State::<2, 2>::new(actors);

        let moves = state.valid_moves();

        let valid_hallway_position = HashSet::from([0, 1, 3, 5, 7, 9, 10]);
        assert_eq!(valid_hallway_position.len(), moves.len());
        for (new_state, cost) in &moves {
            assert!(new_state.valid());
            assert!(new_state.actors[1].position == Position::Room(1, 0));
            match new_state.actors[0].position {
                Position::Hallway(p) => {
                    assert!(valid_hallway_position.contains(&p));
                    let hallway_cost = (1 + (6 as i32 - p as i32).abs()) * 10;
                    assert_eq!(hallway_cost as usize, *cost);
                }
                Position::Room(_, _) => panic!(),
            }
            println!("Cost {}: {:?}", cost, new_state.actors);
            println!("{}\n", new_state.display());
        }

        let actors = [
            Actor {
                target_room: 1,
                position: Position::Room(2, 1),
            },
            Actor {
                target_room: 1,
                position: Position::Room(1, 0),
            },
            Actor {
                target_room: 3,
                position: Position::Hallway(7),
            },
        ];

        let state = State::<3, 2>::new(actors);
        let moves = state.valid_moves();
        let valid_hallway_position = HashSet::from([0, 1, 3, 5]);
        let mut hallway_positions_considered = HashSet::new();
        for (new_state, cost) in &moves {
            if let Position::Hallway(p) = new_state.actors[2].position {
                // Ignore where actor 2 moved
                assert_eq!(Position::Hallway(p), actors[2].position);
            } else {
                continue;
            }
            assert!(new_state.valid());
            assert!(new_state.actors[1].position == Position::Room(1, 0));
            match new_state.actors[0].position {
                Position::Hallway(p) => {
                    assert!(valid_hallway_position.contains(&p));
                    hallway_positions_considered.insert(p);
                    let hallway_cost = (1 + (6 as i32 - p as i32).abs()) * 10;
                    assert_eq!(hallway_cost as usize, *cost);
                }
                Position::Room(_, _) => {
                    println!("{}\n", new_state.display());
                    panic!();
                }
            }
            println!("Cost {}: {:?}", cost, new_state.actors);
            println!("{}\n", new_state.display());
        }
        assert_eq!(hallway_positions_considered, valid_hallway_position);
    }

    #[test]
    fn test_calculate_cost_done() {
        let actors = [
            Actor {
                target_room: 1,
                position: Position::Room(1, 1),
            },
            Actor {
                target_room: 1,
                position: Position::Room(1, 0),
            },
        ];

        let state = State::<2, 2>::new(actors);
        assert!(state.done());
        let (cost, explored) = calculate_cost(state);
        assert_eq!(cost, 0);
        assert_eq!(explored, 1);
    }
    #[test]
    fn test_calculate_cost_hallway_to_room() {
        let actors = [
            Actor {
                target_room: 1,
                position: Position::Hallway(3),
            },
            Actor {
                target_room: 1,
                position: Position::Room(1, 0),
            },
        ];

        let state = State::<2, 2>::new(actors);
        assert_eq!(state.done(), false);

        for m in state.valid_moves() {
            println!("COST: {}\n{}", m.1, m.0.display());
        }

        let (cost, explored) = calculate_cost(state);
        assert_eq!(cost, 20);
        assert_eq!(explored, 2);
    }
}
