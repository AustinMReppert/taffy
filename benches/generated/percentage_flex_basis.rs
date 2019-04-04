pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0.25f32),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
