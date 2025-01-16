use crate::Task;
use unicode_segmentation::UnicodeSegmentation;

const COLUMN_ID_SIZE: usize = 20;
const COLUMN_NAME_SIZE: usize = 20;
const COLUMN_MODIFICATION_SIZE: usize = 12;
const COLUMN_TERM_SIZE: usize = 10;
const COLUMN_TASK_STATE_SIZE: usize = 11;
const COLUMN_PRIORITY_SIZE: usize = 8;
const COLUMN_DESCRIPTION_SIZE: usize = 40;

const HEADER_FIELDS: [&str; 7] = [
    "ID",
    "Name",
    "Modification",
    "Term",
    "Task State",
    "Priority",
    "Description",
];

const HEADER_SIZES: [usize; 7] = [
    COLUMN_ID_SIZE,
    COLUMN_NAME_SIZE,
    COLUMN_MODIFICATION_SIZE,
    COLUMN_TERM_SIZE,
    COLUMN_TASK_STATE_SIZE,
    COLUMN_PRIORITY_SIZE,
    COLUMN_DESCRIPTION_SIZE,
];

pub fn show_query_tasks(tasks: &Vec<Task>) {
    show_divisor();
    show_header();
    show_each_tasks(tasks);
}
fn show_each_tasks(tasks: &Vec<Task>) {
    for i in tasks.iter() {
        show_divisor();
        show_task(&i);
    }
    show_divisor();
}
fn show_header() {
    let mut header: String = String::new();
    for (i, j) in HEADER_FIELDS.iter().enumerate() {
        let padding = HEADER_SIZES[i].saturating_sub(j.len());
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;

        let centered = format!(
            "|{:>widthl$}{}{:>widthr$}",
            "",
            j,
            "",
            widthl = left_padding as usize,
            widthr = right_padding as usize
        );
        header += centered.as_str();
    }
    header += "|";
    println!("{}", header);
}

fn show_divisor() {
    let mut header = String::new();
    for i in HEADER_SIZES {
        let space = format!("|{}", "=".repeat(i as usize));
        header += space.as_str();
    }
    header += "|";
    println!("{}", header);
}

fn show_task(task: &Task) {
    let formated = format_to_square(task);
    let matrix = get_sliced_cells(formated);

    let mut task_info: String = String::new();
    for i in 0..matrix[0].len() {
        for j in 0..matrix.len() {
            task_info += format!("|{}", matrix[j][i].clone()).as_str();
        }
        if i == matrix[0].len() - 1 {
            task_info += "|";
        } else {
            task_info += "|\n";
        }
    }
    println!("{}", task_info);
}

fn get_sliced_cells(vec_task: Vec<String>) -> Vec<Vec<String>> {
    let mut mat: Vec<Vec<String>> = Vec::new();
    for (i, j) in vec_task.iter().enumerate() {
        let words = j
            .graphemes(true)
            .collect::<Vec<_>>()
            .chunks(HEADER_SIZES[i])
            .map(|chunk| {
                let mut word: String = String::new();
                for i in chunk {
                    word += i;
                }
                return word;
            })
            .collect::<Vec<String>>();
        mat.push(words);
    }
    return mat;
}

fn format_to_square(task: &Task) -> Vec<String> {
    let minimal_number = calculate_number_of_lines(task);
    let infos = task.to_vec();
    let mut new_lines: Vec<String> = Vec::new();
    for i in 0..infos.len() {
        let new_info = infos[i].clone()
            + String::from(" ")
                .repeat((minimal_number * HEADER_SIZES[i]) - infos[i].graphemes(true).count())
                .as_str();

        new_lines.push(new_info);
    }
    return new_lines;
}

fn calculate_number_of_lines(task: &Task) -> usize {
    let mut minimal_number = 1;
    let infos = task.to_vec();
    for (i, j) in infos.iter().enumerate() {
        let lines = (j.graphemes(true).count() + HEADER_SIZES[i] - 1) / HEADER_SIZES[i];
        if lines > minimal_number {
            minimal_number = lines;
        }
    }

    return minimal_number;
}
