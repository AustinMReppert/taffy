pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(20f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(30f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(40f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(50f32),
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
