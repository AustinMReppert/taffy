pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(400f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0.15f32),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4f32,
                    flex_basis: stretch::style::Dimension::Percent(0.1f32),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2f32),
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
