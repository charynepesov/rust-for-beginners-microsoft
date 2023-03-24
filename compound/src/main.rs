fn main() {
    let array = [1, 2, 3, 4];
    let first_element = array[0];
    //let element = array[100];

    let len = "some text".len();
    [1][len];

    let array: [u32, 3] = [1u32, 2, true];

    let tuple = (1u32, 2, true);
    let first_element = tuple.0;
    let element = tuple.100;

}
