use std::io;

fn main() {
    println!("Birinci sayıyı girin:");
    let sayi1 = read_number();

    println!("İkinci sayıyı girin:");
    let sayi2 = read_number();

    println!("Bir işlem seçin (+, -, *, /):");
    let mut islem = String::new();
    io::stdin().read_line(&mut islem).expect("İşlem okunamadı.");
    let islem = islem.trim();

    let sonuc = match islem {
        "+" => sayi1 + sayi2,
        "-" => sayi1 - sayi2,
        "*" => sayi1 * sayi2,
        "/" => {
            if sayi2 != 0.0 {
                sayi1 / sayi2
            } else {
                println!("Hata: Sıfıra bölme işlemi yapılamaz!");
                return;
            }
        }
        _ => {
            println!("Hata: Geçersiz işlem seçtiniz!");
            return;
        }
    };

    println!("Sonuç: {}", sonuc);
}

fn read_number() -> f64 {
    loop {
        let mut giris = String::new();
        io::stdin()
            .read_line(&mut giris)
            .expect("Girdi okunamadı.");

        match giris.trim().parse::<f64>() {
            Ok(sayi) => return sayi,
            Err(_) => println!("Lütfen geçerli bir sayı girin!"),
        }
    }
}
