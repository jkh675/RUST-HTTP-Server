use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use rust_http_server::ThreadPool;

const BIND_ADDRESS: &'static str = "127.0.0.1:80";
const DEFAULT_200_PAGE: &'static str = "index.html";
const DEFAULT_404_PAGE: &'static str = "404.html";
const THREADPOOL_SIZE:usize = 100; //每个线程可以同时处理20个请求 60*20 就是可以同时处理1200个请求
const EXPECTED_HEADER: &[u8; 16] = b"GET / HTTP/1.1\r\n";
const BUFFER_SIZE:usize = 1024;


fn main() {
    let listener =TcpListener::bind(BIND_ADDRESS).unwrap();//开始监听TCP流量
    let pool = ThreadPool::new(THREADPOOL_SIZE);
    for stream in listener.incoming() { //开始循环监听
        let stream = stream.unwrap(); //解包流量
        pool.execute(|| {
            process_handle(stream);
        });
    }
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}

fn process_handle(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_SIZE]; //创建缓冲区 // TODO: 可变缓冲大小
    let i = stream.read(&mut buffer); //读取请求
    match i {
        Ok(_) =>{
            let path = String::from_utf8_lossy(&buffer);
            let path: Vec<&str> = path.lines().next().unwrap().split(" ").collect();


            let get = EXPECTED_HEADER; //定义接受的请求
            let (status_line, filename) = //定义返回
            if buffer.starts_with(get) 
            {
                ("HTTP/1.1 200 OK \r\n\r\n",DEFAULT_200_PAGE)
            } else {
                //("HTTP/1.1 404 NOT FOUND \r\n\r\n",path[1])
                ("HTTP/1.1 200 OK \r\n\r\n",rem_first_and_last(path[1]))
            };
            

            let contents = fs::read_to_string(filename); //读取文件
            match contents {
                Ok(contents) =>{
                    let response = format!("{} \r\n\r\n {}", status_line, contents); //构建返回值
                    let a =stream.write(response.as_bytes()); //写入
                    match a{
                        Ok(_) => stream.flush().unwrap(), //发送
                        Err(_) => return,
                    };
                },
                Err(_) => {
                    let strerr = fs::read_to_string(DEFAULT_404_PAGE);
                    match strerr
                    {
                        Ok(_) =>{
                            let strf = format!("HTTP/1.1 404 NOT FOUND \r\n\r\n {}", fs::read_to_string(DEFAULT_404_PAGE).unwrap());
                            let response = strf; //构建返回值
                            let a =stream.write(response.as_bytes()); //写入
                            match a{
                                Ok(_) => stream.flush().unwrap(), //发送
                                Err(_) => return,
                            };
                        },
                        Err(_) => {
                            let strf = format!("HTTP/1.1 404 NOT FOUND \r\n\r\n {}", "404 Not Found");
                            let response = strf; //构建返回值
                            let a =stream.write(response.as_bytes()); //写入
                            match a{
                                Ok(_) => stream.flush().unwrap(), //发送
                                Err(_) => return,
                            };
                        },
                    };
                },
            };
        },
        Err(_) =>{
            return
        },
    };
}