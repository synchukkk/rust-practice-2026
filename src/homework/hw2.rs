use std::process::Command;
use std::fs;
use std::path::Path;

fn get_home() -> String {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| String::from("C:\\Users\\yarik"))
}

fn run_command_output(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd).args(args).output();
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            if !out.status.success() {
                stderr
            } else {
                stdout
            }
        }
        Err(e) => format!("Помилка: {}", e),
    }
}

// ============ КРОК 1: SSH-ключ ============

fn check_ssh_key() {
    let key_path = format!("{}/.ssh/id_ed25519", get_home());
    let pub_path = format!("{}/.ssh/id_ed25519.pub", get_home());

    println!("=== КРОК 1: Перевірка SSH-ключа ===");

    if Path::new(&key_path).exists() {
        println!("✓ SSH-ключ існує: {}", key_path);

        // Показати публічний ключ
        match fs::read_to_string(&pub_path) {
            Ok(key) => println!("✓ Публічний ключ:\n{}", key),
            Err(_) => println!("✗ Не вдалося прочитати публічний ключ"),
        }
    } else {
        println!("✗ SSH-ключ не знайдено. Генерація...");
        let key_path_str = key_path.as_str();
        let result = run_command_output(
            "ssh-keygen",
            &["-t", "ed25519", "-C", "synchukkk@github", "-f", key_path_str, "-N", ""],
        );
        println!("{}", result);
    }
}

// ============ КРОК 2: Перевірка SSH-з'єднання ============

fn test_ssh_github() {
    println!("\n=== КРОК 2: Перевірка SSH-з'єднання з GitHub ===");
    let result = run_command_output("ssh", &["-T", "-o", "StrictHostKeyChecking=no", "git@github.com"]);
    println!("{}", result);
}

// ============ КРОК 3: Клонування через SSH ============

fn clone_via_ssh(repo_url: &str, folder: &str) {
    println!("\n=== КРОК 3: Клонування репозиторію через SSH ===");
    println!("URL: {}", repo_url);

    if Path::new(folder).exists() {
        println!("✓ Папка '{}' вже існує — клонування не потрібне", folder);
    } else {
        let result = run_command_output("git", &["clone", repo_url, folder]);
        println!("{}", result);
        println!("✓ Репозиторій клоновано в папку '{}'", folder);
    }
}

// ============ КРОК 4: Додати файл та зробити push через SSH ============

fn git_push_via_ssh(folder: &str) {
    println!("\n=== КРОК 4: Commit та Push через SSH (без логіну/паролю) ===");

    // Створити тестовий файл
    let file_path = format!("{}/hw2_test.txt", folder);
    fs::write(&file_path, "Практична 2 — push через SSH без логіну та паролю\n")
        .expect("Не вдалося створити файл");
    println!("✓ Файл створено: {}", file_path);

    // git add
    let add = run_command_output("git", &["-C", folder, "add", "."]);
    println!("git add: {}", if add.is_empty() { "✓ OK".to_string() } else { add });

    // git commit
    let commit = run_command_output(
        "git",
        &["-C", folder, "commit", "-m", "hw2: add file via SSH without login/password"],
    );
    println!("git commit: {}", commit);

    // git push через SSH (без логіну та паролю)
    let push = run_command_output("git", &["-C", folder, "push"]);
    println!("git push: {}", if push.is_empty() { "✓ Push успішний!".to_string() } else { push });
}

// ============ КРОК 5: Перевірити remote URL (має бути SSH, не HTTPS) ============

fn check_remote_url(folder: &str) {
    println!("\n=== КРОК 5: Перевірка remote URL ===");
    let result = run_command_output("git", &["-C", folder, "remote", "-v"]);
    println!("{}", result);

    if result.contains("git@github.com") {
        println!("✓ Remote використовує SSH — логін та пароль НЕ потрібні!");
    } else if result.contains("https://") {
        println!("✗ Remote використовує HTTPS — потрібно змінити на SSH!");
        println!("  Виконай: git remote set-url origin git@github.com:synchukkk/rust-practice-2026.git");
    }
}

fn main() {
    let repo_ssh  = "git@github.com:synchukkk/rust-practice-2026.git";
    let folder    = "rust-practice-2026";

    println!("╔══════════════════════════════════════════════╗");
    println!("║   Практична 2 — SSH без логіну та паролю    ║");
    println!("╚══════════════════════════════════════════════╝\n");

    check_ssh_key();
    test_ssh_github();
    clone_via_ssh(repo_ssh, folder);
    check_remote_url(folder);
    git_push_via_ssh(folder);

    println!("\n╔══════════════════════════════════════════════╗");
    println!("║   ✓ Всі кроки виконано через SSH!           ║");
    println!("║   ✓ Без використання логіну та паролю!      ║");
    println!("╚══════════════════════════════════════════════╝");
}
