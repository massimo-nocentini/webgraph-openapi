#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};
use webgraph::{
    prelude::BvGraph,
    traits::{RandomAccessGraph, RandomAccessLabeling, SequentialLabeling},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Neighborhood {
    graph_name: String,
    num_nodes: usize,
    num_arcs: u64,
    vertex_id: usize,
    outdegree: usize,
    neighborhood: Vec<usize>,
}

#[get("/neighborhood/<topic>/<graph_name>/<vertex_id>")]
fn neighborhood(topic: &str, graph_name: &str, vertex_id: usize) -> Json<Neighborhood> {
    let path = format!("/usr/webgraphs/{}/{}", topic, graph_name);
    let graph = BvGraph::with_basename(&path).load().unwrap();

    Json(Neighborhood {
        graph_name: graph_name.to_string(),
        num_nodes: graph.num_nodes(),
        num_arcs: graph.num_arcs(),
        vertex_id,
        neighborhood: graph.successors(vertex_id).collect(),
        outdegree: graph.outdegree(vertex_id),
    })
}

#[get("/")]
fn echo() -> &'static str {
    "Usage: 
    
        GET /webgraph-api/neighborhood/<topic>/<graph_name>/<vertex_id>
            Returns the neighborhood of a vertex in the specified graph under the given topic."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/webgraph-api", routes![neighborhood, echo])
}
