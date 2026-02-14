use core::fmt::Display;

const DEFAULT_ITEMS_DISPLAY: usize = 3;

pub fn fmt_slice_compact<T: Display> (xs: &[T], max_elems_display: Option<usize>) -> String {
    // Case 1: display all
    let mut n_elems = max_elems_display.unwrap_or(DEFAULT_ITEMS_DISPLAY);
    n_elems = n_elems.min(xs.len());
    let ellipse :bool = n_elems < xs.len();

    let take = if n_elems >= 2 { n_elems-1 } else { n_elems };
    let mut parts: Vec<String> = xs[..take].iter().map(|x| x.to_string()).collect();
    if ellipse {
        parts.push("...".into());
    }
    if n_elems - take > 0 {
        // There have room for the last one to display
        parts.push(xs[xs.len() - 1].to_string());
    }

    format!("[{}]", parts.join(", "))
}

#[macro_export]
macro_rules! fmt_slice_debug {
    ($xs:expr) => {{
        $crate::formatx::fmt_slice_compact($xs, None)
    }};

    ($xs:expr, ALL) => {{
        $crate::formatx::fmt_slice_compact($xs, Some(usize::MAX))
    }};

    ($xs:expr, $n:expr) => {{
        $crate::formatx::fmt_slice_compact($xs, Some($n))
    }};
}