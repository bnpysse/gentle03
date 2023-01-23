// Gentle-Intro-Rust, 3th, 文件系统与进程,(越来越难了...)
// 2023年1月22日13时44分42秒

fn main() {
    // ==========再看看读取文件==========  
    println!("\n==========再看看读取文件==========");
    // fs::File 实现了 io::Read ,这是一个具备可读性的trait,这个trait定义了一个能填充 u8 切片字节
    // 的 read 方法,使用 read_to_end 填充可读内容到字节  Vec ,还有 read_to_string 可填充到一个
    // string--必须是 utf-8 编码
    // 下面先实现一个没有缓冲区的原始读取
    // 对于缓冲性读取,有 io::BufRead trait,给了我们 read_line 和一个 lines 迭代器
    // io::BufReader 将给 任何 具备可读性的类型提供 io::BufRead 实现
    // 确保所有这些 trait 可用的最简单方法是: use std::io::prelude::*

    use std::fs::File;
    use std::io;
    use std::io::prelude::*;

    fn read_all_lines_ver_1(filename: &str) -> io::Result<()> {
        let file = File::open(&filename)?;
        let reader = io::BufReader::new(file);
        // lines 作为一个迭代器,可以直接使用collect 从一个文件中读取为一个字符串向量
        // 或者用 enumerate 迭代器打印带行号的 line
        for line in reader.lines() {
            // 迭代器返回的 line 实际上是一个 io::Result<String> ,我们用 ? 解开它,
            // 因为在迭代过程中可能会出现错误,比如 I/O 非utf-8的字节块
            let line = line?;
            println!("{}", line);
        }
        Ok(())
    }
    // 以上非读取 所有行 的最有效方式,因为每行都要分配一个新字符串,有成本
    // 使用 read_line 效率更高
    struct Lines<R> {
        reader: io::BufReader<R>,
        buf: String,
    }
    impl <R: Read> Lines<R> {
        fn new(r: R) -> Lines<R> {
            Lines { reader: io::BufReader::new(r), buf: String::new() }
        }
        fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>> {
            self.buf.clear();
            match self.reader.read_line(&mut self.buf) {
                Ok(nbytes) => if nbytes == 0 {
                    None
                } else {
                    let line = self.buf.trim_right();
                    Some(Ok(line))
                },
                Err(e) => Some(Err(e))
            }
        }
    }
    fn read_all_lines(filename: &str) -> io::Result<()> {
        let file = File::open(&filename)?;
        let mut lines = Lines::new(file);
        while let Some(line) = lines.next() {
            let line = line?;
            println!("{}", line);
        }
        Ok(())
    }

    // ==========写入文件==========
    println!("\n==========写入文件==========");
    fn write_out(f: &str) -> io::Result<()> {
        let mut out = File::create(f)?;
        write!(out, "answer is {}\n", 42)?;
        Ok(())
    }
    write_out("test.txt")
        .expect("test.txt write failed");
    // Rust文件默認是无缓冲的,所以每个小的写入请求都会直接进入操作系统,而这会明显变慢

    // ==========文件,路径和目录==========
    println!("\n==========文件,路径和目录==========");
    // 这是一个用于在机器上打印 Cargo 目录的小程序,在一个 Unix Shell 环境中,使用
    // env::home_dir, 因其是跨平台的
    // 创建一个 PathBuf ,并使用它的 push 方法,构建完整的文件路径就像 组件
    use std::env;
    use std::path::PathBuf;
    let home = env::home_dir().expect("no home directory");
    let mut path = PathBuf::new();
    path.push(home);
    path.push(".cargo");
    if path.is_dir() {
        println!("{}", path.display());
    }
    let mut path = env::current_dir().expect("can't access current directory");
    println!("\nCurrent directory : {}", path.display());
    loop {
        println!("{}", path.display());
        if !path.pop() {
            break;
        }
    }    
    // 某個文件的信息
    use std::path::Path;
    use std::os::unix::fs::PermissionsExt;

    fn get_filt_info(filename: &str) {
        let path = Path::new(&filename);
        match path.metadata() {
            Ok(data) => {
                println!("type {:?}", data.file_type());
                println!("len {}", data.len());
                println!("perm {:?}", data.permissions());
                println!("perm {:x}", data.permissions().mode());
                println!("modified {:?}", data.modified());
            },
            Err(e) => println!("error {:?}", e),
        }
    }
    get_filt_info("test.txt");
}
