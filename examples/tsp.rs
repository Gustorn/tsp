#[macro_use]
extern crate genetic;

use genetic::{Algorithm, Permutation, Problem};
use genetic::crossover::*;
use genetic::selection::*;
use genetic::mutation::*;
use genetic::reinsertion::*;
use genetic::termination::*;
use genetic::tracking::*;


#[derive(Clone)]
pub struct City {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl City {
    pub fn new(name: &str, lat: f64, lon: f64) -> Self {
        City {
            name: name.to_owned(),
            latitude: lat,
            longitude: lon,
        }
    }
}

pub fn print_path(cities: &[City], chromosome: &[usize]) {
    let path = chromosome.iter()
        .map(|c| cities[*c].name.clone())
        .collect::<Vec<_>>();
    println!("Chromosome: {:?}\nDistance: {}",
             chromosome, travel_distance(cities, chromosome));
    print!("Path: {}", path[0]);
    for city in path {
        print!(", {}", city);
    }
    println!("");
}

pub fn travel_distance(cities: &[City], chromosome: &[usize]) -> f64 {
    let mut distance = 0.0;
    let mut previous = &cities[chromosome[0]];

    for g in chromosome.iter().skip(1) {
        let current = &cities[*g];
        distance += distance_between(previous, current);
        previous = current;
    }
    distance
}

fn distance_between(first: &City, second: &City) -> f64 {
    let r = 6371.0;
    let dlat = (second.latitude - first.latitude).to_radians();
    let dlon = (second.longitude - first.longitude).to_radians();

    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin() +
            first.latitude.to_radians().cos() * second.latitude.to_radians().cos() *
            (dlon / 2.0).sin() * (dlon / 2.0).sin();
    let as1 = (1.0 - a).sqrt();
    let c = 2.0 * a.sqrt().atan2(as1);
    r * c
}

fn main() {
    let cities = vec![City::new("Birmingham", 52.486125, -1.890507),
                      City::new("Bristol", 51.460852, -2.588139),
                      City::new("London", 51.512161, -0.116215),
                      City::new("Leeds", 53.803895, -1.549931),
                      City::new("Manchester", 53.478239, -2.258549),
                      City::new("Liverpool", 53.409532, -3.000126),
                      City::new("Hull", 53.751959, -0.335941),
                      City::new("Newcastle", 54.980766, -1.615849),
                      City::new("Carlisle", 54.892406, -2.923222),
                      City::new("Edinburgh", 55.958426, -3.186893),
                      City::new("Glasgow", 55.862982, -4.263554),
                      City::new("Cardiff", 51.488224, -3.186893),
                      City::new("Swansea", 51.624837, -3.94495),
                      City::new("Exeter", 50.726024, -3.543949),
                      City::new("Falmouth", 50.152266, -5.065556),
                      City::new("Canterbury", 51.289406, 1.075802)];

    let tsp_fit = |c: &[usize]| {
        let distance = travel_distance(&cities, c);
        1.0 - distance / 10000.0
    };

    let mut alg = genetic_algorithm!(
         fitness:     &tsp_fit,
         selection:   Tournament::new(0.90, 7),
         crossover:   (EdgeRecombination::new(), rate: 0.8),
         mutation:    (Twors::new(),             rate: 0.1),
         reinsertion: Elitist::new(),
         tracking:    BestSolution::new()
    );

    let problem = Permutation::from(0..cities.len());
    let population = problem.generate_population(1000);
    let tracking = alg.evolve(population, Iterations::new(400));

    print_path(&cities, &tracking.best().0);
}
