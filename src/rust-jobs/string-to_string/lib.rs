use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    let x: String = "ab".to_string();


    sea::sassert!(x == "ab".to_string());
 






    // let mut x: String = "0123".to_string();
    // let mut array: [u8; 4] = [1, 2, 3, 4];

    // if let Some(slice) = unsafe { x.as_bytes_mut().get_mut(0..4) } {
    //     for i in 0..slice.len() {
    //         let char: u8 = i.try_into().unwrap();
    //         slice[i] = char;
    //         array[i] = char;
    //     }
    // }

    // sea::sassert!(x == array.iter().map(|&byte| byte as char).collect::<String>());
    // sea::sassert!(false);
}
