use reqwest::blocking::Client;

fn generate_docstring(problem_number: u32) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://projecteuler.net/problem={}", problem_number);
    let minimal_url = format!("https://projecteuler.net/minimal={}", problem_number);

    // Fetch problem name from HTML
    let html = Client::new().get(&url).send()?.text()?;
    let problem_name = html
        .lines()
        .find(|line| line.contains("<h2>"))
        .ok_or("Could not find problem name")?
        .split_once("</h2>")
        .unwrap()
        .0
        .split_once("<h2>")
        .unwrap()
        .1;

    // Fetch information from minimal version
    let minimal_html = Client::new().get(&minimal_url).send()?.text()?;
    let minimal_info: String = minimal_html
        .lines()
        .map(|line| format!("/// {}", line))
        .collect::<Vec<String>>()
        .join("\n");

    let docstring = format!("/// Problem {problem_number}: [{problem_name}] \n/// \n/// [{problem_name}]: {url} \n/// \n{minimal_info} \n/// \n/// Answer: todo!()\n");

    Ok(docstring)
}

#[cfg(test)]
mod tests {
    use clipboard_win::{formats, set_clipboard};

    use super::*;

    #[test]
    fn generate_doc() {
        let p_no = 14;
        let ds = generate_docstring(p_no).unwrap();
        set_clipboard(formats::Unicode, ds.clone()).expect("To set clipboard");
        print!("{ds}\n");
    }
}
