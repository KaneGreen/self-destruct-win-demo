#![cfg(target_os = "windows")]
fn main() {
    use std::env;
    use std::io::Write;
    use std::process::{Command, Stdio};
    // Only for Windows OS with Powershell 4.0+
    let mut child = Command::new(r#"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe"#)
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        let command_text = 
            format!("Start-Process -FilePath \"C:\\Windows\\System32\\cmd.exe\" -WindowStyle Hidden -ArgumentList \"/C\",\"timeout 1 & del {}\" ; exit",
                env::current_exe()
                .expect("error when obtaining current exe")
                .to_str()
                .expect("error when obtaining current exe")
            );
        stdin
            .write_all(command_text.as_bytes())
            .expect("Failed to write to stdin");
    }
    println!(
        "{:?}",
        child.wait_with_output().expect("Failed to read stdout")
    );
}
