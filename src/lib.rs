pub mod my_house;

#[cfg(test)]
use my_house::roof::my_tiles::repel_rain;

#[test]
pub fn run_the_house () {
    repel_rain();
    my_house::inside::keep_warm();
    my_house::back_yard::grow_grass();

    my_house::front_yard::grow_grass();
    my_house::front_yard::mail_box::receive_mail();
}

#[cfg(test)]
mod tests {
    use crate::run_the_house;

    #[test]
    fn my_test () {
        run_the_house();
        assert_eq!(2+2, 4);
    }
}