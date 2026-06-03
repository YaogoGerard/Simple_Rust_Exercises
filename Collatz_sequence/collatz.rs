fn lenght(mut n:i32)->u32{
    let mut len=1;
    while n>1{
        if n%2==0{
            n=n/2;
            len+=1;
        }else{
            n=(3*n)+1;
            len+=1;
        }
    }
        return len;
}



fn main(){
    let res=lenght(11);
    println!("{res}");
}