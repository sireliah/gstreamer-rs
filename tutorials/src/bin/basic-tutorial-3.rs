extern crate gstreamer as gst;
extern crate glib;
use gst::prelude::*;

fn main() {

    // Initialize GStreamer
    gst::init().unwrap();

    let uri = "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";

    // Create the elements
    let source = gst::ElementFactory::make("uridecodebin", "source")
        .expect("Couldn't create source element.");
    let convert = gst::ElementFactory::make("audioconvert", "convert")
        .expect("Couldn't create convert element.");
    let sink = gst::ElementFactory::make("autoaudiosink", "sink")
        .expect("Couldn't create sink element.");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::new("test-pipeline");

    source.set_property("uri", &glib::Value::from(uri));
    let elements = &[&source, &convert, &sink];

    pipeline.add_many(elements).expect("Couldn't add elements to the pipeline.");
    gst::Element::link_many(elements).expect("Elements could not be linked.");


    let pipeline_clone = pipeline.clone();
    source.connect_pad_added(move |_, src_pad| {

        let pipeline = &pipeline_clone;

        pipeline.set_state(gst::State::Playing);


    });

}