pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_shrink: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0f32),
                    ..Default::default()
                },
                stretch::style::Node {
                    display: stretch::style::Display::None,
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1f32,
                    flex_shrink: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0f32),
                    children: vec![stretch::style::Node {
                        flex_grow: 1f32,
                        flex_shrink: 1f32,
                        flex_basis: stretch::style::Dimension::Percent(0f32),
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_shrink: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0f32),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
