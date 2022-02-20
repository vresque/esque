use bounds::{LowerBound, TwoWayBound, UpperBound};
use esqtest::*;

use crate::debug;

#[esqtest::test]
pub fn test_bounds() {
    check_eq!(*UpperBound::<100>::new(110), 100);
    check_eq!(*UpperBound::<100>::new(90), 90);

    check_eq!(*LowerBound::<-10>::new(-20), -10);
    check_eq!(*LowerBound::<-10>::new(-8), -8);

    check_eq!(*TwoWayBound::<-20, 100>::new(110), 100);
    check_eq!(*TwoWayBound::<-20, 100>::new(-30), -20);
    check_eq!(*TwoWayBound::<-20, 100, 55>::new(-30), 55);
    let bound = TwoWayBound::<-20, 200>::new(120); // = 120
    let snd = TwoWayBound::<-20, 200>::new(130); // = 130
    check_eq!((bound - snd) /* = -10 */, -10);
    let trd = TwoWayBound::<-20, 200>::new(150); // = 150
    check_eq!(bound - trd /* -30 */, -20); /* Bounds must still be active at this point */

    all_good!()
}
