pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                children: vec![stretch::style::Node {
                    justify_content: stretch::style::JustifyContent::Center,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(1f32),
                        ..Default::default()
                    },
                    children: vec![
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(50f32),
                                height: stretch::style::Dimension::Points(50f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(50f32),
                                height: stretch::style::Dimension::Points(50f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
