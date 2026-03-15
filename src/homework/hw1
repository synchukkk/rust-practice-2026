use std::process::Command;
use std::fs;
use std::path::Path;

fn run_command(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect(&format!("Не вдалося виконати команду: {}", cmd));

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        eprintln!("Помилка: {}", stderr);
    } else {
        println!("{}", stdout);
    }

    stdout
}

fn generate_ssh_key(email: &str) {
    let key_path = format!("{}/.ssh/id_ed25519", std::env::var("HOME").unwrap());

    if Path::new(&key_path).exists() {
        println!("SSH-ключ вже існує: {}", key_path);
    } else {
        println!("Генерація SSH-ключа...");
        run_command(
            "ssh-keygen",
            &[
                "-t", "ed25519",
                "-C", email,
                "-f", &key_path,
                "-N", "",  // порожній пароль
            ],
        );
        println!("SSH-ключ успішно згенеровано!");
    }
}

fn start_ssh_agent() {
    println!("Запуск ssh-agent...");
    run_command("bash", &["-c", "eval \"$(ssh-agent -s)\""]);
    
    let key_path = format!("{}/.ssh/id_ed25519", std::env::var("HOME").unwrap());
    println!("Додавання ключа до ssh-agent...");
    run_command("ssh-add", &[&key_path]);
}

fn show_public_key() {
    let key_path = format!("{}/.ssh/id_ed25519.pub", std::env::var("HOME").unwrap());

    println!("\n=== Ваш публічний SSH-ключ ===");
    println!("Скопіюйте його та додайте на GitHub (Settings -> SSH Keys):\n");

    match fs::read_to_string(&key_path) {
        Ok(key) => println!("{}", key),
        Err(_) => eprintln!("Не вдалося прочитати публічний ключ"),
    }
}

fn configure_git(username: &str, email: &str) {
    println!("Налаштування Git...");
    run_command("git", &["config", "--global", "user.name", username]);
    run_command("git", &["config", "--global", "user.email", email]);
    println!("Git налаштовано: {} <{}>", username, email);
}

fn clone_repo(ssh_url: &str) {
    println!("Клонування репозиторію через SSH...");
    // Приклад: git@github.com:synchukkk/rust-practice-2026.git
    run_command("git", &["clone", ssh_url]);
    println!("Репозиторій успішно клоновано!");
}

fn test_ssh_connection() {
    println!("\nПеревірка SSH-з'єднання з GitHub...");
    run_command("ssh", &["-T", "git@github.com"]);
}

fn main() {
    // ====== НАЛАШТУЙ ЦІ ЗНАЧЕННЯ ======
    let email    = "your_email@example.com";
    let username = "your_github_username";
    let ssh_url  = "git@github.com:synchukkk/rust-practice-2026.git";
    // ==================================

    println!("=== Налаштування SSH для GitHub ===\n");

    // Крок 1: Згенерувати SSH-ключ
    generate_ssh_key(email);

    // Крок 2: Запустити ssh-agent і додати ключ
    start_ssh_agent();

    // Крок 3: Показати публічний ключ (треба вручну додати на GitHub)
    show_public_key();

    // Крок 4: Налаштувати Git
    configure_git(username, email);

    // Крок 5: Перевірити з'єднання
    test_ssh_connection();

    // Крок 6: Клонувати репозиторій через SSH
    clone_repo(ssh_url);

    println!("\n=== Готово! Тепер можна працювати без логіну та паролю ===");
}
