#[macro_use]
extern crate rocket;

use rocket::serde::{Serialize, json::Json};
use webgraph::{
    prelude::BvGraph,
    traits::{RandomAccessGraph, RandomAccessLabeling, SequentialLabeling},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GraphSummary {
    graph_name: String,
    num_nodes: usize,
    num_arcs: u64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Neighborhood {
    vertex: usize,
    outborhood: Vec<(usize, usize, usize)>,
    inborhood: Vec<(usize, usize, usize)>,
}

#[get("/summary/<topic>/<graph_name>")]
fn summary(topic: &str, graph_name: &str) -> Json<GraphSummary> {
    let path = format!("/usr/webgraphs/{}/{}", topic, graph_name);
    let graph = BvGraph::with_basename(&path).load().unwrap();

    Json(GraphSummary {
        graph_name: graph_name.to_string(),
        num_nodes: graph.num_nodes(),
        num_arcs: graph.num_arcs(),
    })
}

#[get("/neighborhood/<topic>/<graph_name>/<vertex_id>")]
fn neighborhood(topic: &str, graph_name: &str, vertex_id: usize) -> Json<Neighborhood> {
    let path = format!("/usr/webgraphs/{}/{}", topic, graph_name);
    let graph = BvGraph::with_basename(&path).load().unwrap();

    let path_t = format!("/usr/webgraphs/{}/{}-t", topic, graph_name);
    let graph_t = BvGraph::with_basename(&path_t).load().unwrap();

    Json(Neighborhood {
        vertex: vertex_id,
        outborhood: graph
            .successors(vertex_id)
            .map(|each| (each, graph.outdegree(each), graph_t.outdegree(each)))
            .collect(),
        inborhood: graph_t
            .successors(vertex_id)
            .map(|each| (each, graph.outdegree(each), graph_t.outdegree(each)))
            .collect(),
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
    rocket::build().mount("/webgraph-api", routes![neighborhood, summary, echo])
}
