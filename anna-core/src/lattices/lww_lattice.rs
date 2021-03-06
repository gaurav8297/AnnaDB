use crate::lattices::base_lattices::Lattice;

#[derive(Clone)]
pub struct TimestampValuePair<T> {
    pub(crate) timestamp: u128,
    pub(crate) value: T
}

impl<T> TimestampValuePair<T> {
    #[inline]
    fn new(val: T) -> TimestampValuePair<T> {
        return TimestampValuePair{
            timestamp: 0,
            value: val
        };
    }

    #[inline]
    fn new_with_timestamp(val: T, ts: u128) -> TimestampValuePair<T> {
        return TimestampValuePair{
            timestamp: ts,
            value: val
        };
    }

    // TODO - Figure out why size is required
}

#[derive(Clone)]
pub struct LWWLattice<T> {
    pub(crate) element: TimestampValuePair<T>
}

impl<T: Clone> Lattice<TimestampValuePair<T>> for LWWLattice<T> {
    fn reveal(&self) -> &TimestampValuePair<T> {
        return &self.element;
    }

    fn merge_elem(&mut self, t: &TimestampValuePair<T>) {
        if t.timestamp >= self.element.timestamp {
            self.element.timestamp = t.timestamp;
            self.element.value = t.value.clone();
        }
    }
}
