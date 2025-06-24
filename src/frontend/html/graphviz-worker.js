
importScripts('viz-standalone.js');

const graphviz = {
    instance: null,
};

Viz.instance().then(instance => {
    graphviz.instance = instance;// Load Viz.js instance
});

this.onmessage = function (e) {

    var json_graph = graphviz.instance.renderJSON(e.data.graph, { engine: e.data.engine, yInvert: true });

    postMessage(json_graph)

}