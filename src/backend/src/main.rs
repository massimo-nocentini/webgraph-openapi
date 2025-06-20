#[macro_use]
extern crate rocket;

use std::sync::Arc;

use avgdist_rs::avgdist_sample;
use rocket::serde::{Deserialize, Serialize, json::Json};
use webgraph::{
    prelude::BvGraph,
    traits::{RandomAccessGraph, RandomAccessLabeling, SequentialLabeling},
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AvgdistRequest {
    num_threads: usize,
    sample_size: usize,
    exact: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct AvgdistResponse {
    diameter: usize,
    distance: f64,
}

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

fn _labels<G: RandomAccessGraph>(graph: &G, vertex: usize) -> Vec<usize> {
    let a = RandomAccessLabeling::labels(&graph, vertex);
    a.into_iter().collect::<Vec<usize>>()
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

#[post("/avgdist/<topic>/<graph_name>", data = "<params>")]
fn avgdist(topic: &str, graph_name: &str, params: Json<AvgdistRequest>) -> Json<AvgdistResponse> {
    let path = format!("/usr/webgraphs/{}/{}", topic, graph_name);
    let graph = BvGraph::with_basename(&path).load().unwrap();

    let thread_pool = rayon::ThreadPoolBuilder::default()
        .num_threads(params.num_threads)
        .build()
        .expect("Failed to create thread pool");

    let tuple = avgdist_sample(
        &thread_pool,
        params.sample_size,
        Arc::new(graph),
        params.exact,
    );

    Json(AvgdistResponse {
        diameter: tuple.0,
        distance: tuple.3 / (tuple.4 as f64),
    })
}

#[get("/neighborhood/<topic>/<graph_name>/<vertex_id>")]
fn neighborhood_get(topic: &str, graph_name: &str, vertex_id: usize) -> Json<Vec<Neighborhood>> {
    neighborhood(topic, graph_name, Json(vec![vertex_id]))
}

#[post("/neighborhood/<topic>/<graph_name>", data = "<vertices>")]
fn neighborhood(
    topic: &str,
    graph_name: &str,
    vertices: Json<Vec<usize>>,
) -> Json<Vec<Neighborhood>> {
    let path = format!("/usr/webgraphs/{}/{}", topic, graph_name);
    let graph = BvGraph::with_basename(&path).load().unwrap();

    let path_t = format!("/usr/webgraphs/{}/{}-t", topic, graph_name);
    let graph_t = BvGraph::with_basename(&path_t).load().unwrap();

    Json(
        vertices
            .iter()
            .map(|each| {
                let vertex_id = *each;

                Neighborhood {
                    vertex: vertex_id,
                    // labels: labels(&graph, vertex_id),
                    outborhood: graph
                        .successors(vertex_id)
                        .map(|each| {
                            (
                                each,
                                graph.outdegree(each),
                                graph_t.outdegree(each),
                                // labels(&graph, each),
                            )
                        })
                        .collect(),
                    inborhood: graph_t
                        .successors(vertex_id)
                        .map(|each| {
                            (
                                each,
                                graph.outdegree(each),
                                graph_t.outdegree(each),
                                // labels(&graph, each),
                            )
                        })
                        .collect(),
                }
            })
            .collect(),
    )
}

#[get("/")]
fn echo() -> &'static str {
    "Usage: 
    
        GET /webgraph-api/neighborhood/<topic>/<graph_name>/<vertex_id>
            Returns the neighborhood of a vertex in the specified graph under the given topic."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/webgraph-api",
        routes![avgdist, neighborhood_get, neighborhood, summary, echo],
    )
}
