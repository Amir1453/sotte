#![allow(unused)]

use renderer::{
    TriangleMeshBuilder,
    larp::Boundable,
    math::{MortonCode, MortonEncoder, MortonParameter},
};

pub fn cat() -> TriangleMeshBuilder {
    TriangleMeshBuilder::new()
        .soup_from_obj("assets/cat/cat.obj")
        .scale_translate(0.6, [0., -10., 0.])
}

pub fn lucky() -> TriangleMeshBuilder {
    TriangleMeshBuilder::new()
        .soup_from_obj("assets/lucky.obj")
        .scale_translate(0.1, [0., 20., 0.])
}

pub fn maria() -> TriangleMeshBuilder {
    TriangleMeshBuilder::new()
        .soup_from_obj("assets/maria/Maria_C.obj")
        .scale_translate(15., [0., -25., 0.])
}

pub fn mortonic_cat<T: MortonParameter>() -> Vec<MortonCode<T>> {
    let mesh = cat().build_soup();
    let encoder = MortonEncoder::<T>::new(&mesh.bounding_box());

    mesh.iter()
        .map(|p| encoder.encode(&p.bounding_box().center()))
        .collect()
}

pub fn mortonic_lucky<T: MortonParameter>() -> Vec<MortonCode<T>> {
    let mesh = lucky().build_soup();
    let encoder = MortonEncoder::<T>::new(&mesh.bounding_box());

    mesh.iter()
        .map(|p| encoder.encode(&p.bounding_box().center()))
        .collect()
}

pub fn mortonic_maria<T: MortonParameter>() -> Vec<MortonCode<T>> {
    let mesh = maria().build_soup();
    let encoder = MortonEncoder::<T>::new(&mesh.bounding_box());

    mesh.iter()
        .map(|p| encoder.encode(&p.bounding_box().center()))
        .collect()
}
