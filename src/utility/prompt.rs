use std::fs::File;
use std::io::prelude::*;

pub enum Prompt {
    #[allow(dead_code)]
    Blog,
    BlogFinetuned,
}

pub fn import_and_populate_prompt(
    prompt: Prompt,
    avatar_description: String,
    keyword: String,
) -> std::io::Result<String> {
    // Match prompt ID to prompt TXT path
    let prompt_id = match prompt {
        Prompt::Blog => "blog",
        Prompt::BlogFinetuned => "blog_finetuned",
    };

    // Open prompt file
    let mut prompt_file = File::open(format!("prompts/{}.txt", prompt_id))?;

    // Read contents into string
    let mut file_contents = String::new();
    prompt_file.read_to_string(&mut file_contents)?;

    // Replace tokens with custom prompt content
    file_contents = file_contents.replace("[AVATAR_DESCRIPTION]", &avatar_description);
    file_contents = file_contents.replace("[KEYWORD]", &keyword);

    Ok(file_contents)
}
