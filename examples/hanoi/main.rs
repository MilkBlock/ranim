use glam::dvec3;
use ranim::{
    animation::transform::TransformAnimSchedule,
    color::{HueDirection, palettes::manim},
    components::Anchor,
    items::{group::Group, vitem::Rectangle},
    prelude::*,
    utils::rate_functions::{ease_in_quad, ease_out_quad, linear},
};

fn solve_hanoi(
    n: usize,
    idx_src: usize,
    idx_dst: usize,
    idx_tmp: usize,
    move_disk: &mut impl FnMut(usize, usize),
) {
    if n == 1 {
        move_disk(idx_src, idx_dst);
    } else {
        solve_hanoi(n - 1, idx_src, idx_tmp, idx_dst, move_disk);
        move_disk(idx_src, idx_dst);
        solve_hanoi(n - 1, idx_tmp, idx_dst, idx_src, move_disk);
    }
}

#[scene]
struct HanoiScene(pub usize);

impl TimelineConstructor for HanoiScene {
    fn construct(self, timeline: &RanimTimeline, _camera: &mut Rabject<CameraFrame>) {
        let n = self.0;
        let total_sec = 10.0;
        let rod_width = 0.4;
        let rod_height = 5.0;
        let rod_section_width = 4.0;

        let _rods = timeline.insert([-1, 0, 1].into_iter().map(|i| {
            let mut rod = Rectangle(rod_width, rod_height).build();
            rod.set_color(manim::GREY_C).put_anchor_on(
                Anchor::edge(0, -1, 0),
                dvec3(i as f64 * rod_section_width, -4.0, 0.0),
            );
            rod
        }));

        let min_disk_width = rod_width * 1.7;
        let max_disk_width = rod_section_width * 0.8;
        let disk_height = (rod_height * 0.8) / n as f64;
        let _disks = timeline.insert((0..n).map(|i| {
            let factor = i as f64 / (n - 1) as f64;
            let disk_width = min_disk_width + (max_disk_width - min_disk_width) * (1.0 - factor);
            let mut disk = Rectangle(disk_width, disk_height).build();
            let color = manim::RED_D.lerp(manim::BLUE_D, factor as f32, HueDirection::Increasing);
            disk.set_color(color).set_stroke_width(0.0).put_anchor_on(
                Anchor::edge(0, -1, 0),
                dvec3(-rod_section_width, -4.0 + disk_height * i as f64, 0.0),
            );
            disk
        }));

        let mut disks = [_disks, Group::new(), Group::new()];

        let anim_duration = total_sec / (2.0f64.powi(n as i32) - 1.0) / 3.0;
        let mut move_disk = |idx_src: usize, idx_dst: usize| {
            let top_disk_y = |idx: usize| disks[idx].len() as f64 * disk_height - 4.0;
            let top_src = top_disk_y(idx_src) - disk_height;
            let top_dst = top_disk_y(idx_dst);
            let mut disk = disks[idx_src].pop().unwrap();

            timeline.play(
                disk.transform(|data| {
                    data.shift(dvec3(0.0, 3.0 - top_src, 0.0));
                })
                .with_duration(anim_duration)
                .with_rate_func(ease_in_quad)
                .apply(),
            );
            timeline.play(
                disk.transform(|data| {
                    data.shift(dvec3(
                        (idx_dst as f64 - idx_src as f64) * rod_section_width,
                        0.0,
                        0.0,
                    ));
                })
                .with_duration(anim_duration)
                .with_rate_func(linear)
                .apply(),
            );
            timeline.play(
                disk.transform(|data| {
                    data.shift(dvec3(0.0, top_dst - 3.0, 0.0));
                })
                .with_duration(anim_duration)
                .with_rate_func(ease_out_quad)
                .apply(),
            );
            timeline.sync();
            disks[idx_dst].push(disk);
        };

        solve_hanoi(n, 0, 1, 2, &mut move_disk);
    }
}

fn main() {
    #[cfg(feature = "app")]
    run_scene_app(HanoiScene(10));
    #[cfg(not(feature = "app"))]
    {
        render_scene(
            HanoiScene(5),
            &AppOptions {
                output_filename: "output-5.mp4",
                ..Default::default()
            },
        );
        render_scene_at_sec(HanoiScene(5), 0.0, "preview-5.png", &AppOptions::default());
        render_scene(
            HanoiScene(10),
            &AppOptions {
                output_filename: "output-10.mp4",
                ..Default::default()
            },
        );
        render_scene_at_sec(
            HanoiScene(10),
            0.0,
            "preview-10.png",
            &AppOptions::default(),
        );
    }
}
