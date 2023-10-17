use layout::gv::{DotParser, GraphBuilder};
use layout::backends::svg::SVGWriter;
use layout::core::utils::save_to_file;
use env_logger;
use log;

fn main() {

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None).init();

    let raw_dot = r#"
    digraph G {
        A -> B;
        B -> C;
        C -> D;
        D -> A;
    }
    "#;

    let mut parser = DotParser::new(raw_dot);
    let tree = parser.process();
    log::info!("parse result: {:#?}", &tree);

    if let Ok(g) = tree {
        let mut gb = GraphBuilder::new();
        gb.visit_graph(&g);
        let mut vg = gb.get();

        let mut svg = SVGWriter::new();
        vg.do_it(true, false, false, &mut svg);
        let content = svg.finalize();

        save_to_file("examp.svg", &content).unwrap();
    }
}