//tupla com valores xumbados. Está chumbado também o que será impresso
//ou seja qual dos pontos da tupla será impresso


fn main () {
let numbers: (i32, i32, f64, char, &str) = (23, 43, 6.4, 'a',"Cebonfa");
println! ("{:?}",numbers.2);

}

