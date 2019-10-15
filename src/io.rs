pub fn read_line(url: String) -> String {
    use std::io::{stdin,stdout,Write};
    let mut s = String::new();
    print!(
        "\n[Warning!] attacking the website '{}' can offend the law \
        of your country. \n\nAre you still okay to keep on doing this?\
        [Y/n]: ",
        &url
    );

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");

    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }

    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    if &s == "n" {
        return String::from("");
    }

    return s;
}


