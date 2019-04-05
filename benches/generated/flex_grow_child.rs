pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style { ..Default::default() },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                flex_grow: 1f32,
                flex_basis: stretch::style::Dimension::Points(0f32),
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
