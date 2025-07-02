#![allow(dead_code)]

pub fn find_markers<F>(content: &str, file_name: &str, marker: &str, prompt_fn: F) -> Result<(), std::io::Error>
where 
    F: Fn(&[usize]) -> Result<Vec<usize>, std::io::Error>
{
    let file = std::fs::read_to_string(file_name)?;
    let lines = file.lines().collect::<Vec<&str>>();
    let mut marked_lines = vec![];
    for (i, line) in lines.iter().enumerate() {
        if line.contains(marker) {
            marked_lines.push(i);
        }
    }
    
    let insert_indices = prompt_fn(&marked_lines)?;
    let mut new_lines = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if insert_indices.contains(&i) {
            new_lines.extend(content.lines());
        }
        else { 
            new_lines.push(line);
        }
    }
    std::fs::write(file_name, new_lines.join("\n"))?;
    Ok(())
}