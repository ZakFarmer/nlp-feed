use std::fs::File;
use std::io::prelude::*;

pub enum Prompt {
    Blog,
    News,
}

pub fn import_prompt(prompt: Prompt, keyword: String) -> std::io::Result<String> {
    let prompt_id = match prompt {
        Prompt::Blog => "blog",
        Prompt::News => "news",
        _ => "blog",
    };

    let mut prompt_file = File::open(format!("prompts/{}.txt", prompt_id))?;

    let mut file_contents = String::new();

    prompt_file.read_to_string(&mut file_contents)?;

    file_contents = file_contents.replace("[KEYWORD]", &keyword);

    Ok(file_contents)
}
