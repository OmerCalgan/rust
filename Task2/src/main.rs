use std::io; // Kullanıcı girişi için


/// Matematiksel işlemleri temsil eden enum.
/// Her varyant, işlemin gerektirdiği sayıları tutar.
#[derive(Debug)] // Hata ayıklama çıktısı için türetme
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
    Sqrt(f64), // Yeni: Karekök işlemi için tek bir f64 değeri alır
}

/// Özel hata türümüzü tanımlayan enum.
/// İşlemlerde oluşabilecek hataları daha spesifik hale getiririz.
#[derive(Debug)]
enum CalculatorError {
    DivisionByZero,
    NegativeNumberForSqrt,
    InvalidInput(String),
    UnknownOperation,
}

/// Verilen Operation enum'una göre hesaplama yapar ve sonucu döndürür.
/// Hata durumunda CalculatorError döndüren Result kullanılır.
fn calculate(op: Operation) -> Result<f64, CalculatorError> {
    match op {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        },
        Operation::Sqrt(a) => {
            if a < 0.0 {
                Err(CalculatorError::NegativeNumberForSqrt)
            } else {
                Ok(a.sqrt()) // f64'ün sqrt metodunu kullanırız
            }
        }
    }
}

/// Kullanıcıdan f64 türünde bir sayı girişi alır.
/// Geçersiz giriş durumunda hata mesajı döndürür.
fn get_number_input(prompt: &str) -> Result<f64, CalculatorError> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e| CalculatorError::InvalidInput(format!("Giriş okunamadı: {}", e)))?;

    input.trim().parse::<f64>()
        .map_err(|e| CalculatorError::InvalidInput(format!("Geçersiz sayı: {}", e)))
}

/// Kullanıcıdan operatör girişi alır.
fn get_operator_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Operatör okunamadı!");
    operator.trim().to_string()
}

fn main() {
    println!("--- Rust Hesap Makinesi ---");

    let num1 = match get_number_input("Lütfen ilk sayıyı girin:") {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Hata: {:?}", e);
            return;
        }
    };

    let operator_str = get_operator_input("Lütfen işlemi seçin (+, -, *, /, sqrt):");

    let operation_instance = match operator_str.as_str() {
        "+" => {
            let num2 = match get_number_input("Lütfen ikinci sayıyı girin:") {
                Ok(n) => n,
                Err(e) => { eprintln!("Hata: {:?}", e); return; }
            };
            Operation::Add(num1, num2)
        },
        "-" => {
            let num2 = match get_number_input("Lütfen ikinci sayıyı girin:") {
                Ok(n) => n,
                Err(e) => { eprintln!("Hata: {:?}", e); return; }
            };
            Operation::Subtract(num1, num2)
        },
        "*" => {
            let num2 = match get_number_input("Lütfen ikinci sayıyı girin:") {
                Ok(n) => n,
                Err(e) => { eprintln!("Hata: {:?}", e); return; }
            };
            Operation::Multiply(num1, num2)
        },
        "/" => {
            let num2 = match get_number_input("Lütfen ikinci sayıyı girin:") {
                Ok(n) => n,
                Err(e) => { eprintln!("Hata: {:?}", e); return; }
            };
            Operation::Divide(num1, num2)
        },
        "sqrt" => Operation::Sqrt(num1), // sqrt için sadece ilk sayı yeterli
        _ => {
            eprintln!("Hata: Geçersiz işlem! Desteklenenler: +, -, *, /, sqrt");
            return;
        }
    };

    // calculate fonksiyonunu çağır ve sonucu/hatayı işle
    match calculate(operation_instance) {
        Ok(result) => println!("Sonuç: {}", result),
        Err(e) => eprintln!("İşlem hatası: {:?}", e),
    }

    println!("--- Hesaplama tamamlandı ---");
}