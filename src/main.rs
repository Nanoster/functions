// -------------------function test
// fn main() {
//     // println!("Hello, world!");

//     // another_function(5);
//     print_labeled_measurement(5,'h');
// }

// // fn another_function(x: i32) {
// //     println!("The value of x is: {x}");
// // }

// fn print_labeled_measurement(value: i32,unit_label: char){
//     // println!("The measurement is: );
//     println!("The measurement is: {value}{unit_label}");
// }

//------------------------Statements test
// fn main(){
//     let x = (let y = 6);     //에러 ERROR
// }

//------------------------Expressions test
// fn main(){
//     let y = {//중괄호 안이 표현식, ;(세미콜론)으로 끝나지않음. x+1을 반환 Expressions in this parentheses, It does not end with ;(semicolons), return x+1
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }
//-------------------------function with return values test

// fn five() -> i32 {    //반환값이 i32  Return value type is i32
//     5    //세미콜론이 붙지않았다. It does not end with ;(semicolons)
// }

// fn main(){
//     let x = five();

//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x = plus_one(5);    // 넣은 값을 +1해서 돌려줌: 6. +1 and turn it back: 6

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {    // 매개변수 i32, 반환값 i32. parameter i32, return value i32
//     x + 1    //세미콜론이 붙지않았다. It does not end with ;(semicolons)
// }

fn main() {
    let x = multiple_returns(5);    // 변수의 타입를 지정하지 않으면 오른쪽과 알아서 맞춰서 만들어준다. If you do not specify the type of variable, it is made according to the right.

    println!("The value of x is: {:?}",x);
}

fn multiple_returns(x: i32) -> (i32, i32) {    // 여러개의 반환값을 가지는 경우 튜플형태로 반환. Return in tuple form if you have multiple return values
    let y: (i32,i32) = (x, x+1);    // 선언과 동시에 반환하는 것은 불가능. It is impossible to return it at the same time as a declaration.
    y
}