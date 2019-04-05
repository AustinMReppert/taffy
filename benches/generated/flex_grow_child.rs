pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            children: vec![stretch::style::Node {
                flex_grow: 1f32,
                flex_basis: stretch::style::Dimension::Points(0f32),
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
