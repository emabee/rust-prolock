use crate::s_idx::SIdx;

// TODO: make this a little crate of its own.

// Produces monotonously increasing integer numbers, starting from a configurable start-point.
//
// Can be fast-forwarded to skip numbers.
// Cannot be wound back.
// Will panic if an attempt is done to go beyond the max value.
#[derive(Clone, Default, Debug)]
pub(crate) struct Sequence {
    previous: SIdx,
}
impl Sequence {
    // Next value is 1.
    #[must_use]
    pub fn new() -> Self {
        Self {
            previous: Default::default(),
        }
    }

    // New instance that starts with val + 1.
    #[must_use]
    pub fn from(val: u64) -> Self {
        Self { previous: val }
    }

    // Produce a Sequence that starts after the highest value given.
    pub fn from_highest(values: &mut dyn std::iter::Iterator<Item = &SIdx>) -> Self {
        let mut instance = Self::new();
        for value in values {
            instance.continue_after(*value);
        }
        instance
    }

    // Make sure that the Sequence will never produce the given value,
    // by increasing the latest value if necessary.
    pub fn continue_after(&mut self, val: SIdx) {
        self.previous = std::cmp::max(self.previous, val);
    }

    // Returns the highest value that is considered as given out.
    #[must_use]
    pub fn previous(&self) -> SIdx {
        self.previous
    }

    #[allow(clippy::should_implement_trait)] // FIXME consider implementing std::iter::Iterator
    pub fn next(&mut self) -> SIdx {
        assert!(self.previous < SIdx::MAX, "Sequence is exhausted");
        self.previous += SIdx::from(1_u8);
        self.previous
    }
}

// we use u128 as serialization format as biggest common divisor, leaving room to redefine SIdx
impl serde::ser::Serialize for Sequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u128(self.previous.into())
    }
}
impl<'de> serde::de::Deserialize<'de> for Sequence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = SeqVisitor;
        deserializer.deserialize_u128(visitor)
    }
}
struct SeqVisitor;
impl serde::de::Visitor<'_> for SeqVisitor {
    type Value = Sequence;
    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Sequence::from(SIdx::try_from(value).map_err(E::custom)?))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting a u64")
    }
}

#[cfg(test)]
mod test {
    use std::u64;

    use super::Sequence;

    #[test]
    fn test_sequence() {
        let mut sequence = Sequence::new();
        assert_eq!(sequence.previous(), 0);

        assert_eq!(sequence.next(), 1);

        sequence.continue_after(5);
        assert_eq!(sequence.next(), 6);
        sequence.continue_after(15);
        sequence.continue_after(7);
        sequence.continue_after(0);
        assert_eq!(sequence.next(), 16);
    }

    #[should_panic]
    #[test]
    fn test_exhaust() {
        let mut sequence = Sequence::new();
        sequence.continue_after(u64::MAX);
        let _n = sequence.next();
    }
}
