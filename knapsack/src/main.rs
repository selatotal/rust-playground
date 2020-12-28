use rand::Rng;

const WEIGTHS: &[i32] = &[10, 13, 15, 12, 23, 30, 12];
const CAPACITY: i32 = 50;

fn main() {

    let population_size = 5;
    let mut iteractions = 1;

    let mut population = create_initial_population(population_size);
    println!("Initial Population = {:?}", population);

    loop {
        population = create_population(population.as_mut());
        println!("New Population = {:?}", population);
        if stop(population.as_mut()) { 
            println!("Found Solution in {} iteractions : {:?} - {}!", iteractions, population[0], get_size(&population[0]));
            break;
        }
        iteractions+=1;
    }

}

fn create_initial_population(size: i32) -> Vec<Vec<bool>> {
    let mut population: Vec<Vec<bool>> = Vec::new();
    for _ in 0..size {
        let mut chromossom: Vec<bool> = Vec::new();
        for _ in WEIGTHS {
            chromossom.push(rand::random::<bool>());
        }
        population.push(chromossom);
    }
    return population;
}

fn get_size(chromossom: &Vec<bool>) -> i32 {
    let mut size = 0;
    for i in 0..WEIGTHS.len() {
        if chromossom[i] {
            size += WEIGTHS[i];
        }
    }
    size
}

fn difference(chromossom: &Vec<bool>) -> i32 {
    let size = get_size(&chromossom);
    let difference = size - CAPACITY;
    difference.abs()
}

fn crossover_mutation(chrom_1: &Vec<bool>, chrom_2: &Vec<bool>) -> (Vec<bool>, Vec<bool>) {
    let mut rng = rand::thread_rng();
    let position = rng.gen_range(0, WEIGTHS.len()-1);
    let position_mutation = rng.gen_range(0, WEIGTHS.len()-1);

    let mut child_1: Vec<bool> = Vec::new();
    let mut child_2: Vec<bool> = Vec::new();

    for i in 0..position {
        child_1.push(chrom_1[i]);
        child_2.push(chrom_2[i]);
    }

    for i in position..WEIGTHS.len() {
        child_1.push(chrom_2[i]);
        child_2.push(chrom_1[i]);
    }

    // mutation
    child_1[position_mutation] = !child_1[position_mutation];
    child_2[position_mutation] = !child_2[position_mutation];
    (child_1, child_2)
}

fn stop(population: &mut Vec<Vec<bool>>) -> bool {
    let size = get_size(&population[0]);
    size == CAPACITY
}

fn create_population(population: &mut Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_population: Vec<Vec<bool>> = Vec::new();
    population.sort_by(|a,b| difference(a).cmp(&difference(b)));

    // Elithism of better solution
    new_population.push(population[0].clone());

    // Crossover between better and second
    let cross1 = crossover_mutation(&population[0], &population[1]);
    new_population.push(cross1.0);
    new_population.push(cross1.1);

    // Crossover between second and third
    let cross2 = crossover_mutation(&population[1], &population[2]);
    new_population.push(cross2.0);
    new_population.push(cross2.1);
    new_population
}
