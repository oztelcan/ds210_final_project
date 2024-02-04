use std::fs::File;
use std::io::{self,BufRead};

use std::collections::HashMap;

use itertools::Itertools;


type VertexId = usize;

//Graph struct made with HashMap
struct GraphUndirected {
	vertex_data: HashMap<VertexId,Vec<VertexId>>,
}

impl GraphUndirected {
	fn new()->Self { //Returns empty
		Self {
			vertex_data: HashMap::new()
		}
	}
	
	fn from_ids(ids: &[(VertexId,VertexId)])->Self {
		let mut graph = Self::new();
		
		//Iterate over VertexIds
		//When a new VertexId is found, add it with the corresponding value
		for id in ids {
			if graph.vertex_data.contains_key(&id.0) {
				if !graph.vertex_data.contains_key(&id.1) {
					graph.vertex_data.insert(id.1,Vec::new());
				}
				graph.vertex_data.get_mut(&id.0).unwrap().push(id.1);
			}
			else { //If already exists, add value to vector
				graph.vertex_data.insert(id.0,vec![id.1]);
			}
		}
		
		graph
	}
	
	//Counts neighbors at distance 1
	fn vertices_outneighbors_1(&self)->HashMap<VertexId,usize> {
		HashMap::from_iter(
			self.vertex_data.iter()
				.map(|(key,value)| {
					(*key,value.len())
				})
		)
	}
	
	//Counts neighbors at distance 2
	fn vertices_outneighbors_2(&self)->HashMap<VertexId,usize> {
		HashMap::from_iter(
			self.vertex_data.iter()
				.map(|(key,value)| {
					let num = value.iter()
						.map(|k| {
							self.vertex_data.get(k).unwrap().len()
						})
						.sum();
					
					(*key,num)
				})
		)
	}
}



//Hardcoded path
const EDGES_PATH: &str = "../facebook.0.edges";

fn main() {
	let file = File::open(EDGES_PATH).unwrap();
	
	let mut values: Vec<(VertexId,VertexId)> = Vec::new();
	
	for line_err in io::BufReader::new(file).lines() {
		if let Ok(line) = line_err {
			let mut splitted = line.split(' ');
			let v0 = splitted.nth(0).unwrap().parse::<usize>().unwrap();
			let v1 = splitted.nth(0).unwrap().parse::<usize>().unwrap();
			
			values.push((v0,v1));
		}
	}
	
	let graph = GraphUndirected::from_ids(values.as_slice());
	
	let outneighbors_1 = graph.vertices_outneighbors_1();
	let outneighbors_2 = graph.vertices_outneighbors_2();
	
	let keys_sorted: Vec<&VertexId> = graph.vertex_data.keys().sorted().collect();
	
	//Powers calculated
	let mut powers: Vec<f64> = Vec::new();
	
	println!("Outneighbors of vertices:");
	for key in &keys_sorted {
		println!("\tvertex {}:",key);
		
		let n_1 = outneighbors_1.get(key).unwrap();
		let n_2 = outneighbors_2.get(key).unwrap();
		
		println!("\t\tDegree of neighbors at distance 1: {}",n_1);
		println!("\t\tDegree of neighbors at distance 2: {}",n_2);
		
		//If number of neighbors is one or zero, it is not considered
		if *n_1 > 1 && *n_2 > 1 {
			let power = (*n_2 as f64).log(*n_1 as f64);
			println!("\t\tpower is: {}",power);
			
			powers.push(power);
		}
		else {
			println!("\t\t-");
		}
	}
	
	let mean_power: f64 = powers.iter().sum::<f64>()/powers.len() as f64;
	println!("Mean power of vertices: {}",mean_power);
}
