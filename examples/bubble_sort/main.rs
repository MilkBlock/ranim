use glam::{DVec3, dvec2};
use rand::{SeedableRng, seq::SliceRandom};
use ranim::{
    animation::transform::TransformAnimSchedule, color::palettes::manim, components::Anchor,
    items::vitem::Rectangle, prelude::*, timeline::TimeMark, utils::rate_functions::linear,
};

#[scene]
struct BubbleSortScene(pub usize);

impl TimelineConstructor for BubbleSortScene {
    fn construct(self, timeline: &RanimTimeline, _camera: &mut Rabject<CameraFrame>) {
        let num = self.0;

        let frame_size = dvec2(8.0 * 16.0 / 9.0, 8.0);
        let padded_frame_size = frame_size * 0.9;

        let anim_step_duration = 15.0 / num.pow(2) as f64;

        let width_unit = padded_frame_size.x / num as f64;
        let height_unit = padded_frame_size.y / num as f64;

        let mut rng = rand_chacha::ChaChaRng::seed_from_u64(114514);
        let mut heights = (1..=num)
            .map(|x| x as f64 * height_unit)
            .collect::<Vec<f64>>();
        heights.shuffle(&mut rng);

        let padded_frame_bl = dvec2(padded_frame_size.x / -2.0, padded_frame_size.y / -2.0);
        let mut rects = heights
            .iter()
            .enumerate()
            .map(|(i, &height)| {
                let mut rect = Rectangle(width_unit, height).build();
                let target_bc_coord = padded_frame_bl.extend(0.0)
                    + DVec3::X * (width_unit * i as f64 + width_unit / 2.0);
                rect.scale(DVec3::splat(0.8))
                    .put_anchor_on(Anchor::edge(0, -1, 0), target_bc_coord)
                    .set_color(manim::WHITE)
                    .set_stroke_width(0.0)
                    .set_fill_opacity(0.5);
                timeline.insert(rect)
            })
            .collect::<Vec<_>>();

        let shift_right = DVec3::X * width_unit;
        for i in (1..num).rev() {
            for j in 0..i {
                timeline.play(
                    rects[j]
                        .transform(|data| {
                            data.set_color(manim::BLUE_C).set_fill_opacity(0.5);
                        })
                        .with_duration(anim_step_duration)
                        .with_rate_func(linear)
                        .apply(),
                );
                timeline.play(
                    rects[j + 1]
                        .transform(|data| {
                            data.set_color(manim::BLUE_C).set_fill_opacity(0.5);
                        })
                        .with_duration(anim_step_duration)
                        .with_rate_func(linear)
                        .apply(),
                );
                timeline.sync();

                if heights[j] > heights[j + 1] {
                    timeline.play(
                        rects[j]
                            .transform(|data| {
                                data.shift(shift_right)
                                    .set_color(manim::BLUE_C)
                                    .set_fill_opacity(0.5);
                            })
                            .with_duration(anim_step_duration)
                            .with_rate_func(linear)
                            .apply(),
                    );
                    timeline.play(
                        rects[j + 1]
                            .transform(|data| {
                                data.shift(-shift_right)
                                    .set_color(manim::BLUE_C)
                                    .set_fill_opacity(0.5);
                            })
                            .with_duration(anim_step_duration)
                            .with_rate_func(linear)
                            .apply(),
                    );
                    timeline.sync();
                    heights.swap(j, j + 1);
                    rects.swap(j, j + 1);
                }
                timeline.play(
                    rects[j]
                        .transform(|data| {
                            data.set_color(manim::WHITE).set_fill_opacity(0.5);
                        })
                        .with_duration(anim_step_duration)
                        .with_rate_func(linear)
                        .apply(),
                );
                timeline.play(
                    rects[j + 1]
                        .transform(|data| {
                            data.set_color(manim::WHITE).set_fill_opacity(0.5);
                        })
                        .with_duration(anim_step_duration)
                        .with_rate_func(linear)
                        .apply(),
                );
                timeline.sync();
            }
        }

        timeline.insert_time_mark(
            timeline.duration_secs() / 2.0,
            TimeMark::Capture(format!("preview-{num}.png")),
        );
    }
}

fn main() {
    #[cfg(feature = "app")]
    run_scene_app(BubbleSortScene(100));
    #[cfg(not(feature = "app"))]
    {
        render_scene(
            BubbleSortScene(10),
            &AppOptions {
                output_filename: "output-10.mp4",
                ..Default::default()
            },
        );
        render_scene(
            BubbleSortScene(100),
            &AppOptions {
                output_filename: "output-100.mp4",
                ..Default::default()
            },
        );
    }
}
