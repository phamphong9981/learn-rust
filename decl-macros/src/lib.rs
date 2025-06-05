#[macro_export]
macro_rules! vec_of_strings {
    // Không có tham số
    () => {
        Vec::new()
    };
    // Một hoặc nhiều string literal
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! calculate {
    // Phép cộng: calculate!(+ 1, 2, 3)
    (+ $($x:expr),*) => {
        {
            let mut sum = 0;
            $(
                sum += $x;
            )*
            sum
        }
    };
    // Phép nhân: calculate!(* 1, 2, 3)
    (* $($x:expr),*) => {
        {
            let mut product = 1;
            $(
                product *= $x;
            )*
            product
        }
    };
}

#[macro_export]
macro_rules! create_function {
    // Tạo hàm với tên và nội dung: create_function!(foo, println!("Hello"));
    ($func_name:ident, $func_body:expr) => {
        fn $func_name() {
            $func_body
        }
    };
}

#[macro_export]
macro_rules! log_vars {
    // Log nhiều biến với tên: log_vars!(x, y, z)
    ($($var:ident),*) => {
        $(
            println!("{} = {:?}", stringify!($var), $var);
        )*
    };
}
