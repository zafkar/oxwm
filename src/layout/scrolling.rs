use super::{GapConfig, Layout, WindowGeometry};
use x11rb::protocol::xproto::Window;

pub struct ScrollingLayout;

struct GapValues {
    outer_horizontal: u32,
    outer_vertical: u32,
    inner_vertical: u32,
}

impl ScrollingLayout {
    fn getgaps(gaps: &GapConfig, window_count: usize, smartgaps_enabled: bool) -> GapValues {
        let outer_enabled = if smartgaps_enabled && window_count == 1 {
            0
        } else {
            1
        };

        GapValues {
            outer_horizontal: gaps.outer_horizontal * outer_enabled,
            outer_vertical: gaps.outer_vertical * outer_enabled,
            inner_vertical: gaps.inner_vertical,
        }
    }
}

impl Layout for ScrollingLayout {
    fn name(&self) -> &'static str {
        "scrolling"
    }

    fn symbol(&self) -> &'static str {
        "[>>]"
    }

    fn arrange(
        &self,
        windows: &[Window],
        screen_width: u32,
        screen_height: u32,
        gaps: &GapConfig,
        _master_factor: f32,
        num_master: i32,
        smartgaps_enabled: bool,
    ) -> Vec<WindowGeometry> {
        let window_count = windows.len();
        if window_count == 0 {
            return Vec::new();
        }

        let gap_values = Self::getgaps(gaps, window_count, smartgaps_enabled);

        let outer_horizontal = gap_values.outer_horizontal;
        let outer_vertical = gap_values.outer_vertical;
        let inner_vertical = gap_values.inner_vertical;

        let visible_count = if num_master > 0 {
            num_master as usize
        } else {
            2
        };

        let available_width = screen_width.saturating_sub(2 * outer_vertical);
        let available_height = screen_height.saturating_sub(2 * outer_horizontal);

        let total_inner_gaps = if visible_count > 1 {
            inner_vertical * (visible_count.min(window_count) - 1) as u32
        } else {
            0
        };
        let window_width = if window_count <= visible_count {
            let num_windows = window_count as u32;
            let total_gaps = if num_windows > 1 {
                inner_vertical * (num_windows - 1)
            } else {
                0
            };
            (available_width.saturating_sub(total_gaps)) / num_windows
        } else {
            (available_width.saturating_sub(total_inner_gaps)) / visible_count as u32
        };

        let mut geometries = Vec::with_capacity(window_count);
        let mut x = outer_vertical as i32;

        for _window in windows.iter() {
            geometries.push(WindowGeometry {
                x_coordinate: x,
                y_coordinate: outer_horizontal as i32,
                width: window_width,
                height: available_height,
            });

            x += window_width as i32 + inner_vertical as i32;
        }

        geometries
    }
}
