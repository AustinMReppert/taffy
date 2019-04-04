pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(87.4f32),
                height: stretch::style::Dimension::Points(113.4f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.7f32,
                    flex_basis: stretch::style::Dimension::Points(50.3f32),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.3f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.6f32,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.1f32,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.7f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
