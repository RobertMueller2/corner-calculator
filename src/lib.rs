pub trait DirectionConvertible {
    fn get_2xmod(self) -> Self;
    fn get_2ymod(self) -> Self;
    fn get_new_coord(self, axis: bool, screen: Self, window: Self) -> Self;
    fn get_new_x_coord(self, screen: Self, window: Self) -> Self;
    fn get_new_y_coord(self, screen: Self, window: Self) -> Self;
}

macro_rules! generate_mod_functions {
    ($T: ty) => {
        impl DirectionConvertible for $T {
            fn get_2xmod(self) -> Self {
                (self.clamp(1, 9) - 1) % 3
            }

            fn get_2ymod(self) -> Self {
                2 - (self.clamp(1, 9) - 1) / 3
            }

            fn get_new_coord(
                self,
                axis: bool,
                screen_distance: Self,
                window_distance: Self,
            ) -> Self {
                let double_modifier;
                if (axis) {
                    double_modifier = self.get_2ymod();
                } else {
                    double_modifier = self.get_2xmod();
                }

                let screen_distance = screen_distance.clamp(0, <$T>::MAX / 2);
                let window_distance = window_distance.clamp(0, screen_distance);

                //println!("{:?} {:?} {:?}", screen_distance,window_distance,double_modifier);

                ((screen_distance - window_distance) * double_modifier) / 2
            }

            fn get_new_x_coord(self, screen_distance: Self, window_distance: Self) -> Self {
                self.get_new_coord(false, screen_distance, window_distance)
            }

            fn get_new_y_coord(self, screen_distance: Self, window_distance: Self) -> Self {
                self.get_new_coord(true, screen_distance, window_distance)
            }
        }
    };
}

generate_mod_functions!(u16);
generate_mod_functions!(u32);
generate_mod_functions!(i16);
generate_mod_functions!(i32);

#[test]
fn test_get_xmod() {
    for n in [1, 4, 7] {
        assert_eq!(n.get_2xmod(), 0);
    }

    for n in [2, 5, 8] {
        assert_eq!(n.get_2xmod(), 1);
    }

    for n in [3, 6, 9] {
        assert_eq!(n.get_2xmod(), 2);
    }
}

#[test]
fn test_get_ymod() {
    for n in [7, 8, 9] {
        assert_eq!(n.get_2ymod(), 0);
    }

    for n in [4, 5, 6] {
        assert_eq!(n.get_2ymod(), 1);
    }

    for n in [1, 2, 3] {
        assert_eq!(n.get_2ymod(), 2);
    }
}

#[test]
fn test_get_new_coord() {
    for n in [1, 4, 7] as [u16; 3] {
        println!("test new_x_coord block1 {:?}", n);
        assert_eq!(n.get_new_x_coord(3840, 400), 0);
        assert_eq!(n.get_new_x_coord(u16::MAX, u16::MAX), 0);
        assert_eq!(n.get_new_x_coord(u16::MAX, 0), 0);
    }

    for n in [2, 5, 8] as [u16; 3] {
        println!("test new_x_coord block2 {:?}", n);
        assert_eq!(n.get_new_x_coord(3840, 400), 1720);
        // if window is as big as screen, it needs to be at 0.
        assert_eq!(n.get_new_x_coord(u16::MAX, u16::MAX), 0);
        // screen width needs to be clamped at MAX/2. 50% of that is MAX/4 ;)
        assert_eq!(n.get_new_x_coord(u16::MAX, 0), u16::MAX / 4);
    }

    for n in [3, 6, 9] as [u16; 3] {
        println!("test new_x_coord block3 {:?}", n);
        assert_eq!(n.get_new_x_coord(3840, 400), 3440);
        assert_eq!(n.get_new_x_coord(u16::MAX, u16::MAX), 0);
        assert_eq!(n.get_new_x_coord(u16::MAX, 0), u16::MAX / 2);
    }

    for n in [7, 8, 9] as [u16; 3] {
        println!("test new_y_coord block1 {:?}", n);
        assert_eq!(n.get_new_y_coord(3840, 400), 0);
        assert_eq!(n.get_new_y_coord(u16::MAX, u16::MAX), 0);
        assert_eq!(n.get_new_y_coord(u16::MAX, 0), 0);
    }

    for n in [4, 5, 6] as [u16; 3] {
        println!("test new_y_coord block2 {:?}", n);
        assert_eq!(n.get_new_y_coord(3840, 400), 1720);
        assert_eq!(n.get_new_y_coord(u16::MAX, u16::MAX), 0);
        assert_eq!(n.get_new_y_coord(u16::MAX, 0), u16::MAX / 4);
    }

    for n in [1, 2, 3] as [u16; 3] {
        println!("test new_y_coord block3 {:?}", n);
        assert_eq!(n.get_new_y_coord(3840, 400), 3440);
        assert_eq!(n.get_new_y_coord(u16::MAX, u16::MAX), 0);
        assert_eq!(n.get_new_y_coord(u16::MAX, 0), u16::MAX / 2);
    }
}
