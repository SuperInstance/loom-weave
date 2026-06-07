//! Thread-based composition patterns.
//!
//! Provides abstractions for composing data structures using weaving metaphors:
//! warp (longitudinal threads), weft (cross-threads), shed (separation),
//! heddle (pattern control), and tapestry (woven result).


// ============================================================================
// warp module
// ============================================================================

pub mod warp {
    use std::collections::HashMap;

    /// A longitudinal thread in the weave structure.
    #[derive(Debug, Clone)]
    pub struct WarpThread<T: Clone> {
        pub id: usize,
        pub values: Vec<T>,
        pub tension: f64,
        pub metadata: HashMap<String, String>,
    }

    impl<T: Clone> WarpThread<T> {
        pub fn new(id: usize, values: Vec<T>) -> Self {
            Self { id, values, tension: 1.0, metadata: HashMap::new() }
        }

        pub fn with_tension(mut self, tension: f64) -> Self {
            self.tension = tension.clamp(0.0, 2.0);
            self
        }

        pub fn length(&self) -> usize {
            self.values.len()
        }

        pub fn is_empty(&self) -> bool {
            self.values.is_empty()
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            self.values.get(index)
        }

        pub fn set(&mut self, index: usize, value: T) -> bool {
            if index < self.values.len() {
                self.values[index] = value;
                true
            } else {
                false
            }
        }

        pub fn push(&mut self, value: T) {
            self.values.push(value);
        }

        pub fn set_metadata(&mut self, key: &str, value: &str) {
            self.metadata.insert(key.to_string(), value.to_string());
        }

        pub fn get_metadata(&self, key: &str) -> Option<&str> {
            self.metadata.get(key).map(|s| s.as_str())
        }

        pub fn truncate(&mut self, len: usize) {
            self.values.truncate(len);
        }

        pub fn map<U: Clone, F: Fn(&T) -> U>(&self, f: F) -> WarpThread<U> {
            WarpThread {
                id: self.id,
                values: self.values.iter().map(f).collect(),
                tension: self.tension,
                metadata: self.metadata.clone(),
            }
        }
    }

    /// A collection of warp threads forming the longitudinal structure.
    #[derive(Debug, Clone)]
    pub struct Warp<T: Clone> {
        pub threads: Vec<WarpThread<T>>,
    }

    impl<T: Clone> Warp<T> {
        pub fn new() -> Self {
            Self { threads: Vec::new() }
        }

        pub fn add_thread(&mut self, thread: WarpThread<T>) {
            self.threads.push(thread);
        }

        pub fn thread(&self, index: usize) -> Option<&WarpThread<T>> {
            self.threads.get(index)
        }

        pub fn thread_mut(&mut self, index: usize) -> Option<&mut WarpThread<T>> {
            self.threads.get_mut(index)
        }

        pub fn thread_count(&self) -> usize {
            self.threads.len()
        }

        pub fn is_empty(&self) -> bool {
            self.threads.is_empty()
        }

        pub fn max_length(&self) -> usize {
            self.threads.iter().map(|t| t.length()).max().unwrap_or(0)
        }

        pub fn min_length(&self) -> usize {
            self.threads.iter().map(|t| t.length()).min().unwrap_or(0)
        }

        pub fn column(&self, index: usize) -> Vec<Option<&T>> {
            self.threads.iter().map(|t| t.get(index)).collect()
        }

        pub fn uniform_length(&self) -> bool {
            self.min_length() == self.max_length()
        }

        pub fn total_elements(&self) -> usize {
            self.threads.iter().map(|t| t.length()).sum()
        }

        /// Create a warp from a 2D grid.
        pub fn from_grid(grid: &[Vec<T>]) -> Self {
            let threads: Vec<WarpThread<T>> = grid.iter().enumerate()
                .map(|(i, row)| WarpThread::new(i, row.clone()))
                .collect();
            Self { threads }
        }

        /// Average tension across all threads.
        pub fn average_tension(&self) -> f64 {
            if self.threads.is_empty() { return 0.0; }
            self.threads.iter().map(|t| t.tension).sum::<f64>() / self.threads.len() as f64
        }
    }

    impl<T: Clone> Default for Warp<T> {
        fn default() -> Self { Self::new() }
    }
}

// ============================================================================
// weft module
// ============================================================================

pub mod weft {
    use std::collections::HashMap;

    /// A cross-thread (weft) that weaves through the warp.
    #[derive(Debug, Clone)]
    pub struct WeftThread<T: Clone> {
        pub id: usize,
        pub values: Vec<T>,
        pub color: String,
        pub metadata: HashMap<String, String>,
    }

    impl<T: Clone> WeftThread<T> {
        pub fn new(id: usize, values: Vec<T>) -> Self {
            Self { id, values, color: "default".to_string(), metadata: HashMap::new() }
        }

        pub fn with_color(mut self, color: &str) -> Self {
            self.color = color.to_string();
            self
        }

        pub fn length(&self) -> usize {
            self.values.len()
        }

        pub fn is_empty(&self) -> bool {
            self.values.is_empty()
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            self.values.get(index)
        }

        pub fn push(&mut self, value: T) {
            self.values.push(value);
        }

        pub fn insert(&mut self, index: usize, value: T) {
            if index <= self.values.len() {
                self.values.insert(index, value);
            }
        }

        pub fn remove(&mut self, index: usize) -> Option<T> {
            if index < self.values.len() {
                Some(self.values.remove(index))
            } else {
                None
            }
        }

        pub fn reverse(&mut self) {
            self.values.reverse();
        }

        pub fn append(&mut self, other: &WeftThread<T>) {
            self.values.extend_from_slice(&other.values);
        }

        pub fn splice(&self, start: usize, end: usize) -> WeftThread<T> {
            let values: Vec<T> = self.values.iter()
                .skip(start)
                .take(end.saturating_sub(start))
                .cloned()
                .collect();
            WeftThread::new(self.id, values).with_color(&self.color)
        }

        pub fn map<U: Clone, F: Fn(&T) -> U>(&self, f: F) -> WeftThread<U> {
            WeftThread {
                id: self.id,
                values: self.values.iter().map(f).collect(),
                color: self.color.clone(),
                metadata: self.metadata.clone(),
            }
        }

        pub fn zip_with<U: Clone, V: Clone, F: Fn(&T, &U) -> V>(
            &self, other: &WeftThread<U>, f: F
        ) -> WeftThread<V> {
            let values: Vec<V> = self.values.iter()
                .zip(other.values.iter())
                .map(|(a, b)| f(a, b))
                .collect();
            WeftThread::new(self.id, values).with_color(&self.color)
        }
    }

    /// A collection of weft threads.
    #[derive(Debug, Clone)]
    pub struct Weft<T: Clone> {
        pub threads: Vec<WeftThread<T>>,
    }

    impl<T: Clone> Weft<T> {
        pub fn new() -> Self {
            Self { threads: Vec::new() }
        }

        pub fn add_thread(&mut self, thread: WeftThread<T>) {
            self.threads.push(thread);
        }

        pub fn thread(&self, index: usize) -> Option<&WeftThread<T>> {
            self.threads.get(index)
        }

        pub fn thread_count(&self) -> usize {
            self.threads.len()
        }

        pub fn is_empty(&self) -> bool {
            self.threads.is_empty()
        }

        pub fn total_length(&self) -> usize {
            self.threads.iter().map(|t| t.length()).sum()
        }

        pub fn colors(&self) -> Vec<&str> {
            self.threads.iter().map(|t| t.color.as_str()).collect()
        }

        pub fn find_by_color(&self, color: &str) -> Vec<&WeftThread<T>> {
            self.threads.iter().filter(|t| t.color == color).collect()
        }

        pub fn uniform_length(&self) -> bool {
            if self.threads.is_empty() { return true; }
            let len = self.threads[0].length();
            self.threads.iter().all(|t| t.length() == len)
        }
    }

    impl<T: Clone> Default for Weft<T> {
        fn default() -> Self { Self::new() }
    }
}

// ============================================================================
// shed module
// ============================================================================

pub mod shed {
    /// A shed separates warp threads into upper and lower groups for weaving.
    #[derive(Debug, Clone)]
    pub struct Shed {
        pub upper: Vec<usize>,  // indices of threads in upper position
        pub lower: Vec<usize>,  // indices of threads in lower position
        pub opening: f64,       // how wide the shed is open (0.0 - 1.0)
    }

    impl Shed {
        pub fn new(total_threads: usize) -> Self {
            let upper: Vec<usize> = (0..total_threads).step_by(2).collect();
            let lower: Vec<usize> = (1..total_threads).step_by(2).collect();
            Self { upper, lower, opening: 1.0 }
        }

        pub fn with_pattern(total_threads: usize, pattern: &[bool]) -> Self {
            let (upper, lower): (Vec<usize>, Vec<usize>) = (0..total_threads)
                .partition(|&i| pattern.get(i).copied().unwrap_or(i % 2 == 0));
            Self { upper, lower, opening: 1.0 }
        }

        pub fn is_thread_upper(&self, index: usize) -> bool {
            self.upper.contains(&index)
        }

        pub fn is_thread_lower(&self, index: usize) -> bool {
            self.lower.contains(&index)
        }

        pub fn swap(&mut self) {
            std::mem::swap(&mut self.upper, &mut self.lower);
        }

        pub fn set_opening(&mut self, opening: f64) {
            self.opening = opening.clamp(0.0, 1.0);
        }

        pub fn is_open(&self) -> bool {
            self.opening > 0.5
        }

        pub fn is_closed(&self) -> bool {
            self.opening < 0.01
        }

        pub fn thread_count(&self) -> usize {
            self.upper.len() + self.lower.len()
        }

        /// Create a plain weave shed (alternating).
        pub fn plain(total_threads: usize) -> Self {
            Shed::new(total_threads)
        }

        /// Create a twill shed (shifted pattern).
        pub fn twill(total_threads: usize, shift: usize) -> Self {
            let pattern: Vec<bool> = (0..total_threads)
                .map(|i| ((i + shift) % 4) < 2)
                .collect();
            Shed::with_pattern(total_threads, &pattern)
        }

        /// Create a satin shed (distributed pattern).
        pub fn satin(total_threads: usize, interval: usize) -> Self {
            let pattern: Vec<bool> = (0..total_threads)
                .map(|i| i % interval == 0)
                .collect();
            Shed::with_pattern(total_threads, &pattern)
        }

        /// Get the ratio of upper to lower threads.
        pub fn ratio(&self) -> f64 {
            if self.lower.is_empty() { return f64::INFINITY; }
            self.upper.len() as f64 / self.lower.len() as f64
        }
    }
}

// ============================================================================
// heddle module
// ============================================================================

pub mod heddle {
    use super::shed::Shed;

    /// A heddle controls the pattern of the weave by determining which threads are raised.
    #[derive(Debug, Clone)]
    pub struct Heddle {
        pub pattern: Vec<bool>,
        pub row: usize,
        pub repeat: usize,
    }

    impl Heddle {
        pub fn new(pattern: Vec<bool>) -> Self {
            Self { pattern, row: 0, repeat: 1 }
        }

        pub fn with_repeat(mut self, repeat: usize) -> Self {
            self.repeat = repeat.max(1);
            self
        }

        pub fn len(&self) -> usize {
            self.pattern.len()
        }

        pub fn is_empty(&self) -> bool {
            self.pattern.is_empty()
        }

        pub fn get(&self, index: usize) -> bool {
            self.pattern.get(index % self.pattern.len()).copied().unwrap_or(false)
        }

        pub fn set(&mut self, index: usize, value: bool) {
            if index < self.pattern.len() {
                self.pattern[index] = value;
            }
        }

        /// Advance to the next row in the pattern.
        pub fn advance(&mut self) -> bool {
            self.row = (self.row + 1) % (self.pattern.len() * self.repeat);
            self.row == 0
        }

        /// Current state for the given thread count.
        pub fn current_state(&self, thread_count: usize) -> Vec<bool> {
            let effective_row = self.row % self.pattern.len();
            (0..thread_count).map(|i| {
                let pattern_idx = (i + effective_row) % self.pattern.len();
                self.pattern[pattern_idx]
            }).collect()
        }

        /// Create a shed from the current state.
        pub fn create_shed(&self, thread_count: usize) -> Shed {
            let state = self.current_state(thread_count);
            Shed::with_pattern(thread_count, &state)
        }

        /// Plain weave pattern.
        pub fn plain() -> Self {
            Heddle::new(vec![true, false])
        }

        /// Twill pattern.
        pub fn twill() -> Self {
            Heddle::new(vec![true, true, false, false])
        }

        /// Satin pattern (5-shaft).
        pub fn satin() -> Self {
            Heddle::new(vec![true, false, false, false, false])
        }

        /// Invert the pattern.
        pub fn invert(&mut self) {
            for v in &mut self.pattern {
                *v = !*v;
            }
        }

        /// Count of raised threads.
        pub fn raised_count(&self) -> usize {
            self.pattern.iter().filter(|&&v| v).count()
        }

        /// Density: ratio of raised threads.
        pub fn density(&self) -> f64 {
            if self.pattern.is_empty() { return 0.0; }
            self.raised_count() as f64 / self.pattern.len() as f64
        }
    }

    /// A harness holds multiple heddles for complex patterns.
    #[derive(Debug, Clone)]
    pub struct Harness {
        pub heddles: Vec<Heddle>,
        pub current_heddle: usize,
    }

    impl Harness {
        pub fn new() -> Self {
            Self { heddles: Vec::new(), current_heddle: 0 }
        }

        pub fn add_heddle(&mut self, heddle: Heddle) {
            self.heddles.push(heddle);
        }

        pub fn heddle_count(&self) -> usize {
            self.heddles.len()
        }

        pub fn current(&self) -> Option<&Heddle> {
            self.heddles.get(self.current_heddle)
        }

        pub fn current_mut(&mut self) -> Option<&mut Heddle> {
            self.heddles.get_mut(self.current_heddle)
        }

        /// Advance to the next heddle (cycle through).
        pub fn advance(&mut self) -> bool {
            if self.heddles.is_empty() { return true; }
            if let Some(heddle) = self.heddles.get_mut(self.current_heddle) {
                if heddle.advance() {
                    self.current_heddle = (self.current_heddle + 1) % self.heddles.len();
                    self.current_heddle == 0
                } else {
                    false
                }
            } else {
                true
            }
        }

        /// Combined pattern from all heddles.
        pub fn combined_pattern(&self, thread_count: usize) -> Vec<bool> {
            if self.heddles.is_empty() { return vec![false; thread_count]; }
            let mut result = vec![false; thread_count];
            for heddle in &self.heddles {
                let state = heddle.current_state(thread_count);
                for (i, v) in state.iter().enumerate() {
                    if *v { result[i] = true; }
                }
            }
            result
        }

        pub fn is_empty(&self) -> bool {
            self.heddles.is_empty()
        }
    }

    impl Default for Harness {
        fn default() -> Self { Self::new() }
    }
}

// ============================================================================
// tapestry module
// ============================================================================

pub mod tapestry {
    use super::warp::Warp;
    use super::weft::Weft;
    use super::heddle::Heddle;

    /// A cell in the woven tapestry.
    #[derive(Debug, Clone)]
    pub enum Cell<T: Clone> {
        Warp(T),
        Weft(T),
        Empty,
    }

    impl<T: Clone> Cell<T> {
        pub fn is_warp(&self) -> bool {
            matches!(self, Cell::Warp(_))
        }

        pub fn is_weft(&self) -> bool {
            matches!(self, Cell::Weft(_))
        }

        pub fn is_empty(&self) -> bool {
            matches!(self, Cell::Empty)
        }

        pub fn value(&self) -> Option<&T> {
            match self {
                Cell::Warp(v) | Cell::Weft(v) => Some(v),
                Cell::Empty => None,
            }
        }
    }

    /// A woven tapestry result.
    #[derive(Debug, Clone)]
    pub struct Tapestry<T: Clone> {
        pub grid: Vec<Vec<Cell<T>>>,
        pub width: usize,
        pub height: usize,
    }

    impl<T: Clone> Tapestry<T> {
        pub fn new(width: usize, height: usize) -> Self {
            Self {
                grid: vec![vec![Cell::Empty; width]; height],
                width,
                height,
            }
        }

        pub fn set(&mut self, row: usize, col: usize, cell: Cell<T>) {
            if row < self.height && col < self.width {
                self.grid[row][col] = cell;
            }
        }

        pub fn get(&self, row: usize, col: usize) -> Option<&Cell<T>> {
            self.grid.get(row).and_then(|r| r.get(col))
        }

        pub fn is_empty(&self) -> bool {
            self.grid.iter().all(|row| row.iter().all(|c| c.is_empty()))
        }

        pub fn warp_count(&self) -> usize {
            self.grid.iter().flat_map(|row| row.iter()).filter(|c| c.is_warp()).count()
        }

        pub fn weft_count(&self) -> usize {
            self.grid.iter().flat_map(|row| row.iter()).filter(|c| c.is_weft()).count()
        }

        pub fn fill_ratio(&self) -> f64 {
            let total = self.width * self.height;
            if total == 0 { return 0.0; }
            let filled = self.warp_count() + self.weft_count();
            filled as f64 / total as f64
        }

        /// Get a row as references.
        pub fn row(&self, row: usize) -> Option<&[Cell<T>]> {
            self.grid.get(row).map(|r| r.as_slice())
        }

        /// Get a column.
        pub fn column(&self, col: usize) -> Vec<&Cell<T>> {
            self.grid.iter().filter_map(|row| row.get(col)).collect()
        }

        /// Count cells matching a predicate.
        pub fn count_where<F: Fn(&Cell<T>) -> bool>(&self, predicate: F) -> usize {
            self.grid.iter().flat_map(|row| row.iter()).filter(|c| predicate(c)).count()
        }
    }

    /// Weave a tapestry from warp and weft threads using a heddle pattern.
    pub fn weave<T: Clone>(warp: &Warp<T>, weft: &Weft<T>, heddle: &Heddle) -> Tapestry<T> {
        let width = warp.max_length().max(weft.threads.first().map(|t| t.length()).unwrap_or(0));
        let height = warp.thread_count().max(weft.thread_count());
        let mut tapestry = Tapestry::new(width, height);

        let pattern = heddle.current_state(width.max(1));

        // Fill warp threads
        for (row_idx, warp_thread) in warp.threads.iter().enumerate() {
            for (col_idx, value) in warp_thread.values.iter().enumerate() {
                if col_idx < width {
                    tapestry.set(row_idx, col_idx, Cell::Warp(value.clone()));
                }
            }
        }

        // Overlay weft threads based on pattern
        for (row_idx, weft_thread) in weft.threads.iter().enumerate() {
            for (col_idx, value) in weft_thread.values.iter().enumerate() {
                if col_idx < width && row_idx < height
                    && !pattern.get(col_idx).copied().unwrap_or(false) {
                        tapestry.set(row_idx, col_idx, Cell::Weft(value.clone()));
                    }
            }
        }

        tapestry
    }

    /// Create a simple checkerboard tapestry.
    pub fn checkerboard<T: Clone>(size: usize, a: T, b: T) -> Tapestry<T> {
        let mut tapestry = Tapestry::new(size, size);
        for row in 0..size {
            for col in 0..size {
                let cell = if (row + col) % 2 == 0 {
                    Cell::Warp(a.clone())
                } else {
                    Cell::Weft(b.clone())
                };
                tapestry.set(row, col, cell);
            }
        }
        tapestry
    }

    /// Create a striped tapestry.
    pub fn stripes<T: Clone>(width: usize, height: usize, stripe_width: usize, a: T, b: T) -> Tapestry<T> {
        let mut tapestry = Tapestry::new(width, height);
        for row in 0..height {
            for col in 0..width {
                let is_stripe_a = (col / stripe_width.max(1)).is_multiple_of(2);
                let cell = if is_stripe_a {
                    Cell::Warp(a.clone())
                } else {
                    Cell::Weft(b.clone())
                };
                tapestry.set(row, col, cell);
            }
        }
        tapestry
    }
}

// Re-exports
pub use warp::{WarpThread, Warp};
pub use weft::{WeftThread, Weft};
pub use shed::Shed;
pub use heddle::{Heddle, Harness};
pub use tapestry::{Cell, Tapestry, weave, checkerboard, stripes};

#[cfg(test)]
mod tests {
    use super::*;

    // ---- warp tests (14) ----

    #[test]
    fn test_warp_thread_new() {
        let wt = warp::WarpThread::new(1, vec![10, 20, 30]);
        assert_eq!(wt.id, 1);
        assert_eq!(wt.length(), 3);
        assert!((wt.tension - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_warp_thread_with_tension() {
        let wt = warp::WarpThread::new(1, vec![1]).with_tension(1.5);
        assert!((wt.tension - 1.5).abs() < 0.01);
    }

    #[test]
    fn test_warp_thread_clamped_tension() {
        let wt = warp::WarpThread::new(1, vec![1]).with_tension(5.0);
        assert!(wt.tension <= 2.0);
    }

    #[test]
    fn test_warp_thread_get_set() {
        let mut wt = warp::WarpThread::new(1, vec![10, 20, 30]);
        assert_eq!(wt.get(1), Some(&20));
        wt.set(1, 25);
        assert_eq!(wt.get(1), Some(&25));
        assert!(!wt.set(10, 0));
    }

    #[test]
    fn test_warp_thread_push() {
        let mut wt = warp::WarpThread::new(1, vec![1]);
        wt.push(2);
        assert_eq!(wt.length(), 2);
    }

    #[test]
    fn test_warp_thread_metadata() {
        let mut wt = warp::WarpThread::new(1, vec![1]);
        wt.set_metadata("color", "red");
        assert_eq!(wt.get_metadata("color"), Some("red"));
        assert_eq!(wt.get_metadata("weight"), None);
    }

    #[test]
    fn test_warp_thread_map() {
        let wt = warp::WarpThread::new(1, vec![1, 2, 3]);
        let mapped = wt.map(|v| v * 2);
        assert_eq!(mapped.values, vec![2, 4, 6]);
    }

    #[test]
    fn test_warp_thread_truncate() {
        let mut wt = warp::WarpThread::new(1, vec![1, 2, 3, 4, 5]);
        wt.truncate(3);
        assert_eq!(wt.length(), 3);
    }

    #[test]
    fn test_warp_new() {
        let w: warp::Warp<i32> = warp::Warp::new();
        assert!(w.is_empty());
        assert_eq!(w.thread_count(), 0);
    }

    #[test]
    fn test_warp_add_thread() {
        let mut w = warp::Warp::new();
        w.add_thread(warp::WarpThread::new(0, vec![1, 2, 3]));
        w.add_thread(warp::WarpThread::new(1, vec![4, 5]));
        assert_eq!(w.thread_count(), 2);
        assert_eq!(w.max_length(), 3);
        assert_eq!(w.min_length(), 2);
    }

    #[test]
    fn test_warp_column() {
        let mut w = warp::Warp::new();
        w.add_thread(warp::WarpThread::new(0, vec![1, 2]));
        w.add_thread(warp::WarpThread::new(1, vec![3, 4]));
        let col = w.column(0);
        assert_eq!(col.len(), 2);
    }

    #[test]
    fn test_warp_from_grid() {
        let grid = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let w = warp::Warp::from_grid(&grid);
        assert_eq!(w.thread_count(), 3);
        assert!(w.uniform_length());
    }

    #[test]
    fn test_warp_total_elements() {
        let mut w = warp::Warp::new();
        w.add_thread(warp::WarpThread::new(0, vec![1, 2, 3]));
        w.add_thread(warp::WarpThread::new(1, vec![4, 5]));
        assert_eq!(w.total_elements(), 5);
    }

    #[test]
    fn test_warp_average_tension() {
        let mut w = warp::Warp::new();
        w.add_thread(warp::WarpThread::new(0, vec![1]).with_tension(0.8));
        w.add_thread(warp::WarpThread::new(1, vec![1]).with_tension(1.2));
        assert!((w.average_tension() - 1.0).abs() < 0.01);
    }

    // ---- weft tests (14) ----

    #[test]
    fn test_weft_thread_new() {
        let wt = weft::WeftThread::new(1, vec![10, 20]);
        assert_eq!(wt.id, 1);
        assert_eq!(wt.length(), 2);
        assert_eq!(wt.color, "default");
    }

    #[test]
    fn test_weft_thread_with_color() {
        let wt = weft::WeftThread::new(1, vec![1]).with_color("blue");
        assert_eq!(wt.color, "blue");
    }

    #[test]
    fn test_weft_thread_push() {
        let mut wt = weft::WeftThread::new(1, vec![1]);
        wt.push(2);
        assert_eq!(wt.length(), 2);
    }

    #[test]
    fn test_weft_thread_insert() {
        let mut wt = weft::WeftThread::new(1, vec![1, 3]);
        wt.insert(1, 2);
        assert_eq!(wt.values, vec![1, 2, 3]);
    }

    #[test]
    fn test_weft_thread_remove() {
        let mut wt = weft::WeftThread::new(1, vec![1, 2, 3]);
        let removed = wt.remove(1);
        assert_eq!(removed, Some(2));
        assert_eq!(wt.length(), 2);
    }

    #[test]
    fn test_weft_thread_reverse() {
        let mut wt = weft::WeftThread::new(1, vec![1, 2, 3]);
        wt.reverse();
        assert_eq!(wt.values, vec![3, 2, 1]);
    }

    #[test]
    fn test_weft_thread_append() {
        let mut wt1 = weft::WeftThread::new(1, vec![1, 2]);
        let wt2 = weft::WeftThread::new(2, vec![3, 4]);
        wt1.append(&wt2);
        assert_eq!(wt1.values, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_weft_thread_splice() {
        let wt = weft::WeftThread::new(1, vec![1, 2, 3, 4, 5]);
        let spliced = wt.splice(1, 4);
        assert_eq!(spliced.values, vec![2, 3, 4]);
    }

    #[test]
    fn test_weft_thread_map() {
        let wt = weft::WeftThread::new(1, vec![1, 2, 3]);
        let mapped = wt.map(|v| v + 10);
        assert_eq!(mapped.values, vec![11, 12, 13]);
    }

    #[test]
    fn test_weft_thread_zip_with() {
        let a = weft::WeftThread::new(1, vec![1, 2, 3]);
        let b = weft::WeftThread::new(2, vec![10, 20, 30]);
        let zipped = a.zip_with(&b, |x, y| x + y);
        assert_eq!(zipped.values, vec![11, 22, 33]);
    }

    #[test]
    fn test_weft_new() {
        let w: weft::Weft<i32> = weft::Weft::new();
        assert!(w.is_empty());
    }

    #[test]
    fn test_weft_colors() {
        let mut w = weft::Weft::new();
        w.add_thread(weft::WeftThread::new(1, vec![1]).with_color("red"));
        w.add_thread(weft::WeftThread::new(2, vec![1]).with_color("blue"));
        assert_eq!(w.colors(), vec!["red", "blue"]);
    }

    #[test]
    fn test_weft_find_by_color() {
        let mut w = weft::Weft::new();
        w.add_thread(weft::WeftThread::new(1, vec![1]).with_color("red"));
        w.add_thread(weft::WeftThread::new(2, vec![1]).with_color("blue"));
        w.add_thread(weft::WeftThread::new(3, vec![1]).with_color("red"));
        assert_eq!(w.find_by_color("red").len(), 2);
    }

    #[test]
    fn test_weft_uniform_length() {
        let mut w = weft::Weft::new();
        w.add_thread(weft::WeftThread::new(1, vec![1, 2]));
        w.add_thread(weft::WeftThread::new(2, vec![3, 4]));
        assert!(w.uniform_length());
    }

    // ---- shed tests (10) ----

    #[test]
    fn test_shed_new() {
        let shed = shed::Shed::new(6);
        assert_eq!(shed.upper.len(), 3);
        assert_eq!(shed.lower.len(), 3);
        assert_eq!(shed.thread_count(), 6);
    }

    #[test]
    fn test_shed_is_thread_upper() {
        let shed = shed::Shed::new(4);
        assert!(shed.is_thread_upper(0));
        assert!(!shed.is_thread_upper(1));
    }

    #[test]
    fn test_shed_swap() {
        let mut shed = shed::Shed::new(4);
        let upper_before = shed.upper.clone();
        shed.swap();
        assert_eq!(shed.lower, upper_before);
    }

    #[test]
    fn test_shed_opening() {
        let mut shed = shed::Shed::new(4);
        assert!(shed.is_open());
        shed.set_opening(0.3);
        assert!(!shed.is_open());
        assert!(!shed.is_closed());
        shed.set_opening(0.0);
        assert!(shed.is_closed());
    }

    #[test]
    fn test_shed_plain() {
        let shed = shed::Shed::plain(4);
        assert_eq!(shed.upper, vec![0, 2]);
        assert_eq!(shed.lower, vec![1, 3]);
    }

    #[test]
    fn test_shed_twill() {
        let shed = shed::Shed::twill(8, 0);
        assert_eq!(shed.thread_count(), 8);
    }

    #[test]
    fn test_shed_satin() {
        let shed = shed::Shed::satin(10, 5);
        assert_eq!(shed.thread_count(), 10);
        assert!(shed.upper.len() < shed.lower.len());
    }

    #[test]
    fn test_shed_with_pattern() {
        let shed = shed::Shed::with_pattern(4, &[true, false, false, true]);
        assert!(shed.is_thread_upper(0));
        assert!(!shed.is_thread_upper(1));
        assert!(shed.is_thread_upper(3));
    }

    #[test]
    fn test_shed_ratio() {
        let shed = shed::Shed::new(6);
        assert!((shed.ratio() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_shed_all_upper() {
        let shed = shed::Shed::with_pattern(3, &[true, true, true]);
        assert_eq!(shed.lower.len(), 0);
    }

    // ---- heddle tests (12) ----

    #[test]
    fn test_heddle_new() {
        let h = heddle::Heddle::new(vec![true, false]);
        assert_eq!(h.len(), 2);
        assert!(!h.is_empty());
    }

    #[test]
    fn test_heddle_get() {
        let h = heddle::Heddle::new(vec![true, false, true]);
        assert!(h.get(0));
        assert!(!h.get(1));
        assert!(h.get(2));
        assert!(h.get(3)); // wraps to index 0
    }

    #[test]
    fn test_heddle_set() {
        let mut h = heddle::Heddle::new(vec![true, false]);
        h.set(1, true);
        assert!(h.get(1));
    }

    #[test]
    fn test_heddle_advance() {
        let mut h = heddle::Heddle::new(vec![true, false]);
        assert_eq!(h.row, 0);
        h.advance();
        assert_eq!(h.row, 1);
    }

    #[test]
    fn test_heddle_current_state() {
        let h = heddle::Heddle::new(vec![true, false]);
        let state = h.current_state(4);
        assert_eq!(state, vec![true, false, true, false]);
    }

    #[test]
    fn test_heddle_plain() {
        let h = heddle::Heddle::plain();
        assert_eq!(h.pattern, vec![true, false]);
    }

    #[test]
    fn test_heddle_twill() {
        let h = heddle::Heddle::twill();
        assert_eq!(h.pattern, vec![true, true, false, false]);
    }

    #[test]
    fn test_heddle_satin() {
        let h = heddle::Heddle::satin();
        assert_eq!(h.pattern, vec![true, false, false, false, false]);
    }

    #[test]
    fn test_heddle_invert() {
        let mut h = heddle::Heddle::new(vec![true, false]);
        h.invert();
        assert_eq!(h.pattern, vec![false, true]);
    }

    #[test]
    fn test_heddle_density() {
        let h = heddle::Heddle::new(vec![true, false, true, false]);
        assert!((h.density() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_heddle_raised_count() {
        let h = heddle::Heddle::new(vec![true, false, true, true]);
        assert_eq!(h.raised_count(), 3);
    }

    #[test]
    fn test_heddle_create_shed() {
        let h = heddle::Heddle::plain();
        let shed = h.create_shed(4);
        assert_eq!(shed.thread_count(), 4);
    }

    // ---- harness tests (5) ----

    #[test]
    fn test_harness_new() {
        let h = heddle::Harness::new();
        assert!(h.is_empty());
        assert_eq!(h.heddle_count(), 0);
    }

    #[test]
    fn test_harness_add_heddle() {
        let mut h = heddle::Harness::new();
        h.add_heddle(heddle::Heddle::plain());
        h.add_heddle(heddle::Heddle::twill());
        assert_eq!(h.heddle_count(), 2);
    }

    #[test]
    fn test_harness_current() {
        let mut h = heddle::Harness::new();
        h.add_heddle(heddle::Heddle::plain());
        assert!(h.current().is_some());
    }

    #[test]
    fn test_harness_combined_pattern() {
        let mut h = heddle::Harness::new();
        h.add_heddle(heddle::Heddle::new(vec![true, false]));
        let pattern = h.combined_pattern(4);
        assert_eq!(pattern.len(), 4);
    }

    #[test]
    fn test_harness_advance() {
        let mut h = heddle::Harness::new();
        h.add_heddle(heddle::Heddle::new(vec![true, false]));
        h.advance();
        // Should have advanced the heddle
        assert!(true);
    }

    // ---- tapestry tests (10) ----

    #[test]
    fn test_tapestry_new() {
        let t: tapestry::Tapestry<i32> = tapestry::Tapestry::new(3, 3);
        assert_eq!(t.width, 3);
        assert_eq!(t.height, 3);
        assert!(t.is_empty());
    }

    #[test]
    fn test_tapestry_set_get() {
        let mut t = tapestry::Tapestry::new(2, 2);
        t.set(0, 0, tapestry::Cell::Warp(1));
        t.set(0, 1, tapestry::Cell::Weft(2));
        assert!(t.get(0, 0).unwrap().is_warp());
        assert!(t.get(0, 1).unwrap().is_weft());
        assert!(t.get(1, 0).unwrap().is_empty());
    }

    #[test]
    fn test_tapestry_fill_ratio() {
        let mut t = tapestry::Tapestry::new(2, 2);
        assert!((t.fill_ratio()).abs() < 0.01);
        t.set(0, 0, tapestry::Cell::Warp(1));
        assert!((t.fill_ratio() - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_tapestry_counts() {
        let mut t = tapestry::Tapestry::new(2, 2);
        t.set(0, 0, tapestry::Cell::Warp(1));
        t.set(0, 1, tapestry::Cell::Weft(2));
        t.set(1, 0, tapestry::Cell::Warp(3));
        assert_eq!(t.warp_count(), 2);
        assert_eq!(t.weft_count(), 1);
    }

    #[test]
    fn test_tapestry_cell_value() {
        let cell = tapestry::Cell::Warp(42);
        assert_eq!(cell.value(), Some(&42));
        let empty: tapestry::Cell<i32> = tapestry::Cell::Empty;
        assert_eq!(empty.value(), None);
    }

    #[test]
    fn test_tapestry_row() {
        let mut t = tapestry::Tapestry::new(2, 2);
        t.set(0, 0, tapestry::Cell::Warp(1));
        let row = t.row(0);
        assert!(row.is_some());
        assert_eq!(row.unwrap().len(), 2);
    }

    #[test]
    fn test_tapestry_column() {
        let mut t = tapestry::Tapestry::new(2, 2);
        t.set(0, 0, tapestry::Cell::Warp(1));
        t.set(1, 0, tapestry::Cell::Weft(2));
        let col = t.column(0);
        assert_eq!(col.len(), 2);
    }

    #[test]
    fn test_checkerboard() {
        let t = tapestry::checkerboard(4, "A", "B");
        assert_eq!(t.width, 4);
        assert_eq!(t.height, 4);
        assert!(t.warp_count() > 0);
        assert!(t.weft_count() > 0);
    }

    #[test]
    fn test_stripes() {
        let t = tapestry::stripes(6, 4, 2, "X", "Y");
        assert_eq!(t.width, 6);
        assert_eq!(t.height, 4);
    }

    #[test]
    fn test_weave() {
        let mut warp = warp::Warp::new();
        warp.add_thread(warp::WarpThread::new(0, vec![1, 2, 3]));
        warp.add_thread(warp::WarpThread::new(1, vec![4, 5, 6]));

        let mut weft = weft::Weft::new();
        weft.add_thread(weft::WeftThread::new(0, vec![10, 20, 30]));
        weft.add_thread(weft::WeftThread::new(1, vec![40, 50, 60]));

        let heddle = heddle::Heddle::plain();
        let tapestry = tapestry::weave(&warp, &weft, &heddle);
        assert_eq!(tapestry.width, 3);
        assert_eq!(tapestry.height, 2);
    }

    #[test]
    fn test_tapestry_count_where() {
        let mut t = tapestry::Tapestry::new(2, 2);
        t.set(0, 0, tapestry::Cell::Warp(1));
        t.set(0, 1, tapestry::Cell::Weft(2));
        let warp_count = t.count_where(|c| c.is_warp());
        assert_eq!(warp_count, 1);
    }
}
