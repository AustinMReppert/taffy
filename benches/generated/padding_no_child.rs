pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10f32),
                end: stretch::style::Dimension::Points(10f32),
                top: stretch::style::Dimension::Points(10f32),
                bottom: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
