use std::io::Write;
use std::process::{Command, Stdio};

/// Dummy snippets 
const DUMMY_ITEMS: &str = "\
install a package          | sudo pacman -S <package>
search for a package       | pacman -Ss <query>
update all packages        | sudo pacman -Syu
remove a package           | sudo pacman -Rns <package>
list installed packages    | pacman -Q
show disk usage            | df -h
find large files           | find / -size +100M -type f 2>/dev/null
check open ports           | ss -tulnp
show system info           | uname -a
watch log file             | tail -f /var/log/syslog";

/// Spawn fzf with dummy data. Returns the selected line or None if cancelled.
pub fn pick() -> Option<String> {
    let mut child = Command::new("fzf")
        .args([
            "--height=40%",
            "--layout=reverse",
            "--border",
            "--prompt=naive> ",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn fzf — is it installed?");

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(DUMMY_ITEMS.as_bytes());
    }

    let output = child.wait_with_output().expect("failed to read fzf output");

    if output.status.success() {
        let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if selection.is_empty() {
            None
        } else {
            Some(selection)
        }
    } else {
        None
    }
}

/// Extract the command portion from a "description | command" line.
pub fn extract_command(line: &str) -> &str {
    match line.split_once('|') {
        Some((_, cmd)) => cmd.trim(),
        None => line.trim(),
    }
}
