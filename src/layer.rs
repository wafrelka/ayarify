pub type Level = std::num::NonZeroU8;

struct Context<T> {
    stack: Vec<(T, Level)>,
}

impl<T> Context<T> {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn last(&self) -> Option<&(T, Level)> {
        self.stack.last()
    }

    fn push(&mut self, item: (T, Level)) {
        if let Some(last) = self.stack.last() {
            assert!(last.1 < item.1);
        }
        self.stack.push(item);
    }

    fn sweep(&mut self, level: Level) {
        while self.stack.last().map_or(false, |c| c.1 >= level) {
            self.stack.pop();
        }
    }
}

pub fn compute_layer_parents(levels: impl IntoIterator<Item = Option<Level>>) -> Vec<usize> {
    let mut parents = vec![];
    let mut stack = Context::new();

    for (index, level) in levels.into_iter().enumerate() {
        if let Some(level) = level {
            stack.sweep(level);
        }

        let parent = match stack.last() {
            Some((i, _)) => *i,
            None => index,
        };
        parents.push(parent);

        if let Some(level) = level {
            stack.push((index, level));
        }
    }

    parents
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_layer_parents() {
        let level = |n: u8| -> Option<Level> { Some(Level::new(n).unwrap()) };
        let input =
            [level(2), None, level(4), None, level(3), None, level(3), None, level(4), None];
        let expected = vec![0, 0, 0, 2, 0, 4, 0, 6, 6, 8];
        let actual = compute_layer_parents(input.iter().copied());
        assert_eq!(actual, expected);
    }
}
