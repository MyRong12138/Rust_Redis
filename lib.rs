use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
//
pub enum RedisResult {
    RString(String),
    RArr(Vec<String>),
}
//定义一个Resp指令
struct CommandWriter {
    buf: String
}

impl CommandWriter {
    pub fn new() -> CommandWriter {
        CommandWriter { 
            buf: "".to_string(), 
        }
    }
    //resp构建
    // 如果是数组
    fn write_arrs(&mut self, n: usize) -> &mut Self {
        self.add_char('*');
        self.add_uint(n);
        self.add_crnl();
        self
    }

    // 如果是字符串
    fn write_buik_string(&mut self, s: &str) -> &mut Self {
        if s == "" {
            // Null Bulk String
            self.add_str("$-1\r\n");
            return self
        } else {
            self.add_char('$');
            self.add_uint(s.len());
            self.add_crnl();
            self.add_str(s);
            self.add_crnl();
            self
        }    
    }

    #[allow(dead_code)]
    fn write_int(&mut self, n: usize) -> &mut Self {
        self.add_char(':');
        self.add_uint(n);
        self.add_crnl();
        self
    }
    
    //构建协议指令内容
    fn add_char(&mut self, s: char) {
        self.buf.push(s);
    }

    fn add_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    fn add_uint(&mut self, n: usize) {
       self.add_str(n.to_string().as_str());
    }

    fn add_crnl(&mut self) {
        self.add_char('\r');
        self.add_char('\n');
    }    
    
}
//解析resp内容
fn parse_io(response: &str) -> Option<RedisResult> {                                                                                                                        
    let vec: Vec<&str> = response.split("\r\n").collect();
    match &vec[0][0..1] {
        "$" => return Some(RedisResult::RString(vec[1].to_string())),
        "*" => {
            let mut len = vec[0][1..].parse::<usize>().unwrap();
            let mut v: Vec<String> = Vec::new();
            len =len*2;
            for i in 0..len {
                if(i%2!=0)
                {
                v.push(vec[i + 1].to_string());
                }
            }
            println!("长度为{}",len);
            println!("第一个值为{}",v[0]);
            return Some(RedisResult::RArr(v));
        }
        "+" => return Some(RedisResult::RString(vec[1].to_string())),
        "-" => panic!(vec[0].to_string()),
        _ => return None,
    }
}



//建立一个tcp连接
pub struct RedisClient {
    io: TcpStream
}

impl RedisClient {
    pub fn new(sock_addr: &str) -> RedisClient {
        let tcp_strem = TcpStream::connect(sock_addr).unwrap();
        RedisClient {
            io : tcp_strem
        }
    }
//创建redis String内容
    pub fn set(&mut self, key: &str, val: &str) {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(3)
            .write_buik_string("SET")
            .write_buik_string(key)
            .write_buik_string(val);


        self.io.write(cmd.buf.as_bytes()).unwrap();
        self.io.flush().unwrap();
        
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..]).unwrap();
    
    }

//读取redis String内容
    pub fn get(mut self, key: &str) -> String {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(2)
            .write_buik_string("GET")
            .write_buik_string(key);

        self.io.write(cmd.buf.as_bytes()).unwrap();
        
        self.io.flush().unwrap();
        
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..]).unwrap();
    

        let response = str::from_utf8(&buffer).unwrap();

        let parse = parse_io(response).unwrap();
        
        match parse {
            RedisResult::RString(parse) => return parse.to_string(),
            _ => panic!("error")
        }
        
    }

    //创建一个hash
    pub fn setHash(&mut self,hashName: &str,key: &str, val: &str) {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(4)
            .write_buik_string("HSET")
            .write_buik_string(hashName)
            .write_buik_string(key)
            .write_buik_string(val);


        self.io.write(cmd.buf.as_bytes()).unwrap();
        self.io.flush().unwrap();
        
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..]).unwrap();
    
    }

    //获取一个Hash
    pub fn getHash(mut self,hashName: &str, key: &str) -> String {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(3)
            .write_buik_string("HGET")
            .write_buik_string(hashName)
            .write_buik_string(key);

        self.io.write(cmd.buf.as_bytes()).unwrap();
        
        self.io.flush().unwrap();
        
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..]).unwrap();
    

        let response = str::from_utf8(&buffer).unwrap();

        let parse = parse_io(response).unwrap();
        
        match parse {
            RedisResult::RString(parse) => return parse.to_string(),
            _ => panic!("error")
        }
        
    }
    //写入一个列表，没有就创建//完成
    pub fn setList(&mut self,listName: &str,strList:Vec<&str>) {
        let mut cmd = CommandWriter::new();
        let leng=strList.len();
        println!("{}",leng);
        cmd.write_arrs(2+leng)
            .write_buik_string("RPUSH")
            .write_buik_string(listName);
        for i in 0..leng {
            cmd.write_buik_string(strList[i]);
        }

        self.io.write(cmd.buf.as_bytes()).unwrap();
        self.io.flush().unwrap();
        
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..]).unwrap();
    
    }
    //读取一个列表
    pub fn getList(mut self,listName: &str) -> Vec<String> {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(4)
            .write_buik_string("LRANGE")
            .write_buik_string(listName)
            .write_buik_string("0")
            .write_buik_string("100");

        self.io.write(cmd.buf.as_bytes()).unwrap();
        
        self.io.flush().unwrap();
        
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..]).unwrap();
    

        let response = str::from_utf8(&buffer).unwrap();

        let parse = parse_io(response).unwrap();
        
/*
        match parse {
            RedisResult::RString(parse) => return parse,
            _ => response.to_string()
        }
        */
        
                //*
        match parse {
            RedisResult::RArr(parse) => return parse,
            _ => panic!("error")
        }
        //*/
    }
}

