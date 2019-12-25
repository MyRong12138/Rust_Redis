use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

//extern crate simple_redis;
//use simple_redis::Commands;

extern crate redis;
use redis::Commands;

//extern crate simple_redis;
//use simple_redis::simple_redis;

extern crate hello_rust;
use hello_rust::RedisClient;

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;
    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
    println!("开始");
    //fetch_an_integer();
   // println!("{}", redis);
   
   //*
   //不用
   let sock_addr: &str = "127.0.0.1:6379";
   let mut client = RedisClient::new(sock_addr);
   
   //string
   //client.set("stringName", "stringDate");
   //println!("{}", client.get("stringName"));

   //hash
   //client.setHash("name","key","date");
   //println!("获取的值为：{}", client.getHash("name","key"));

   //list
   //let strList = vec!["1", "2", "3", "4", "5"];
   //client.setList("listName",strList);
   //let listDate: Vec<String>=client.getList("listName");
   //for i in listDate {
   // println!("获取的值为：{}", i);
//}
   //println!("获取的值为：{}", client.getList("listName"));

   //无限循环
   loop {
    //println!("again!");
}

   //*/
  //用
  //let mut clients=simple_redis::create("redis://127.0.0.1:6379/");
  //clients.auth("root");
  //let str=clients.get::<String>("test");
  //println(str);
}
/*
fn test() -> Result<(), simple_redis::RedisError> {
    let mut clients=simple_redis::create("redis://127.0.0.1:6379/")?;
    clients.auth("root");
    let str=clients.get::<String>("test")?;
}
*/
/*
fn fetch_an_integer() -> redis::RedisResult<isize> {
    // 连接到redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    let _ : () = con.set("root", 42)?;
    con.get("test")
}
*/