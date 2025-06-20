# Webgraph openapi

This project has the goal to abstract out the powerful encoding and capabilities of [Webgraph](https://github.com/vigna/webgraph-rs)
(see also the companion [paper](https://dl.acm.org/doi/10.1145/3589335.3651581)) via `GET` requests, in order to have a machine readable
data to be used by user in their favourites programming languages.

Our architecture is *micro services*-oriented, namely we rely on containers that play together to ship functionalities as a whole computational unit.
It is composed of two services:

1. a backend written in Rust that accepts HTTP requests, loads a graph using `Webgraph` and responds according to the given query. In particular:
    - a summary of the graph in terms of the count of vertices and edges.
    - given a node, it fetches its *out* and *in* neighbors and forward back to the client.
2. a frontend (a reverse proxy, indeed) that serves a web page that looks like a dashboard that allows the user to explore the desired graph. In particular:
    - it shows some summary data;
    - it exposes a control pane that allows the user to decide if he/she wants to keep objects to the visualization, according to the desired visiting strategy;
    - it shows the topology of the graph, dynamically adding vertices and edges.

Under `src/docker` there is a `Makefile` that exposes some rules to build and get the system up; eventually, `make refresh` should let you head to `http://localhost:8800/`, enjoy!

## API overview

|endpoint|method|body|response|
|---|---|---|---|
|`/webgraph-api/summary/<topic>/<graph_name>`|`GET`||An object with properties `graph_name` of type `str`, `num_nodes` and `num_arcs` of type `usize`. For example, `{"graph_name":"pg","num_nodes":668261953,"num_arcs":2162523341}`.|
|`/webgraph-api/neighborhood/<topic>/<graph_name>/<vertex_id>`|`GET`||An array of objects, each with properties `vertex` of type `usize`, `outborhood` and `inborhood` of type `array` of triples `(v, o, i)` where `v` is the neighbor vertex according to the direction, `o` is its *outdegree* and `i` is its *indegree*, respectively. For example, `[{"vertex":3517367,"outborhood":[[3518074,2,1],[3518075,2,1]],"inborhood":[[3514838,2,1]]}]`.|
|`/webgraph-api/neighborhood/<topic>/<graph_name>`|`POST`|An array of `usize` values, each one is a vertex id in the graph. For example: `[5748,4589,324,1]`|As above, for example `[{"vertex":3517367,"outborhood":[[3518074,2,1],[3518075,2,1]],"inborhood":[[3514838,2,1]]},{"vertex":857487,"outborhood":[[857878,2,1],[857879,2,1]],"inborhood":[[857231,2,1]]}]`.|
|`/webgraph-api/avgdist/<topic>/<graph_name>`|`POST`|An object with properties `num_threads` of type `usize`, `sample_size` of type `usize` and `exact` of type `bool`. For example: `{ "num_threads": 4, "sample_size": 100, "exact": true }`|An object with properties `distance` of type `f64` and `diameter` of type `usize`. For example, `{"distance":351.7367,"diameter":12}`.|