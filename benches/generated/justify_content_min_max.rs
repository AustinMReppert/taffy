pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100f32), ..Default::default() },
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60f32),
                    height: stretch::style::Dimension::Points(60f32),
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
