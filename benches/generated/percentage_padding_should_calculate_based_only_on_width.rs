pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1f32,
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10f32),
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.1f32),
                    end: stretch::style::Dimension::Percent(0.1f32),
                    top: stretch::style::Dimension::Percent(0.1f32),
                    bottom: stretch::style::Dimension::Percent(0.1f32),
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
