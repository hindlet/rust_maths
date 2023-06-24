use rust_maths::*;
use std::f32::EPSILON;


#[test]
fn normal_dist_tests() {
    assert!(0.048394144 - EPSILON < normal_probabilty_density(7.0, 12.0, 5.0) && normal_probabilty_density(7.0, 12.0, 5.0) < 0.048394144 + EPSILON);
    assert!(0.017205188 - EPSILON < normal_probabilty_density(56.0, 70.0, 15.0) && normal_probabilty_density(56.0, 70.0, 15.0) < 0.017205188 + EPSILON);
    assert!(0.002215924 - EPSILON < normal_probabilty_density(24.0, 18.0, 2.0) && normal_probabilty_density(24.0, 18.0, 2.0) < 0.002215924 + EPSILON);
    assert!(0.056068762 - EPSILON < normal_probabilty_density(19.2, 15.0, 5.0) && normal_probabilty_density(19.2, 15.0, 5.0) < 0.056068762 + EPSILON);
}

#[test]
fn lerp_tests() {
    // zero
    assert_eq!(lerp(1.0, 3.0, 0.0), 1.0);
    // one
    assert_eq!(lerp(1.0, 3.0, 1.0), 3.0);
    // surplus negative
    assert_eq!(lerp(1.0, 3.0, -3.0), 1.0);
    // surplus positive
    assert_eq!(lerp(1.0, 3.0, 5.0), 3.0);
    // half
    assert_eq!(lerp(1.0, 3.0, 0.5), 2.0);
    // quarter
    assert_eq!(lerp(1.0, 3.0, 0.25), 1.5);
}


#[cfg(test)]
mod quicksort_tests {
    use super::quicksort;
    #[test]
    fn random_test() {
        let list = vec![(25.6, 0),(22.7, 1),(9.8, 2),(1.5, 3),(5.0, 4),(4.7, 5)];
        assert_eq!(quicksort(list), vec![(1.5, 3),(4.7, 5),(5.0, 4),(9.8, 2),(22.7, 1),(25.6, 0)]);
    }

    #[test]
    fn reverse_test() {
        let list = vec![(25.6, 0),(22.7, 1),(9.8, 2),(5.0, 4),(4.7, 5),(1.5, 3)];
        assert_eq!(quicksort(list), vec![(1.5, 3),(4.7, 5),(5.0, 4),(9.8, 2),(22.7, 1),(25.6, 0)]);
    }

    #[test]
    fn pre_sorted_test() {
        let list = vec![(1.5, 3),(4.7, 5),(5.0, 4),(9.8, 2),(22.7, 1),(25.6, 0)];
        assert_eq!(quicksort(list.clone()), list);
    }
}

#[cfg(test)]
mod quadratic_tests {
    use super::solve_quadratic;
    #[test]
    fn no_real_solutions_test() {
        assert_eq!(solve_quadratic(1.0, 1.0, 8.0), (None, None))
    }

    #[test]
    fn one_real_solution_test() {
        assert_eq!(solve_quadratic(1.0, -8.0, 16.0), (Some(4.0), None))
    }

    #[test]
    fn two_real_solutions_test() {
        assert_eq!(solve_quadratic(1.0, -5.0, 6.0), (Some(3.0), Some(2.0)))
    }
}
