use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::Read;

fn main(){
    let v = vec!["test1.txt", "test2.txt", "test3.txt"];
    doc_to_table(v.clone(), "output1.html");
    doc_to_text_table(v.clone(), "output2.html")
}

#[test]
fn test_doc() {
    let fox = "The quick brown fox jumps over the lazy dog."; 
    let para3 = "a\n\nb\n\nc"; 
    let bustle = r"\ 
        The bustle in a house\n\ 
        The morning after death\n\ 
        Is solemnest of industries\n\ 
        Enacted upon earth,—\n\ 
        \n\ 
        The sweeping up the heart,\n\ 
        And putting love away\n\ 
        We shall not want to use again\n\ 
        Until eternity.\n\ 
    "; 

    let doc1 = make_document(fox);       // 1 paragraph 
    let doc2 = make_document(bustle);    // 2 paragraphs 
    let doc3 = make_document(para3);     // 3 paragraphs 

    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()]; 
    let rnk_docs = rank_documents(&docs); 
    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}

fn make_document(text: &str) -> String {
    text.replace("\\n\\", " ").replace("\\", "").replace("\n          \n", "\n\n")
}

fn rank_documents(v: &Vec<String>) -> Vec<String> {
    let mut vsort = v.clone();
    vsort.sort_by(|x,y| y.matches("\n\n").count().cmp(&x.matches("\n\n").count()));
    vsort
}

#[allow(unused)]
fn doc_to_table(v: Vec<&str>, output_file: &str) {
    let mut docs: Vec<String> = Vec::new();
    for i in v {
        let mut data_file = File::open(i).unwrap();
        let mut file_content = String::new();
        data_file.read_to_string(&mut file_content).unwrap();
        docs.push(file_content.replace("\r\n        \r\n", "\r\n\r\n"));
    }

    docs.sort_by(|x,y| y.matches("\r\n\r\n").count().cmp(&x.matches("\r\n\r\n").count()));
    
    let mut file = File::create(output_file).expect("Failed");
    file.write(b"<style>");
    file.write(b"\ntable, td, th {\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}\n</style>\n");
    file.write(b"<table>\n");
    file.write(b"\t<tr>\n\t\t<th>text</th>\n\t\t<th>number of paragraph</th>\n\t</tr>");
    for i in docs {
        let mut count_line = 0;
        let line = i
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '\n')
            .map(|(i,_)| i)
            .collect::<Vec<_>>();
        if !line.is_empty() {
            for i in 0..line.len()-1 {
                if line[i+1] - line[i] <= 2 {
                    count_line += 1;
                }
            }
        }
        file.write(
            format!(
                "\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>",
                i,
                count_line + 1
            )
            .as_bytes(),
        );
    }
    file.write(b"\n</table>");
}

#[allow(unused)]
fn doc_to_text_table(v: Vec<&str>, output_file: &str) {
    let mut docs: Vec<String> = Vec::new();
    for i in v {
        let mut data_file = File::open(i).unwrap();
        let mut file_content = String::new();
        data_file.read_to_string(&mut file_content).unwrap();
        docs.push(file_content)
    }

    docs.sort_by(|x,y| y.split_ascii_whitespace().count().cmp(&x.split_ascii_whitespace().count()));

    let mut file = File::create(output_file).expect("Failed");
    file.write(b"<style>");
    file.write(b"\ntable, td, th {\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}\n</style>\n");
    file.write(b"<table>\n");
    file.write(b"\t<tr>\n\t\t<th>text</th>\n\t\t<th>number of paragraph</th>\n\t</tr>");
    for i in docs {
        let word = i
            .split_ascii_whitespace()
            .count();
        println!("{:?}", i.split_ascii_whitespace().collect::<Vec<_>>());
        file.write(
            format!(
                "\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>",
                i,
                word
            )
            .as_bytes(),
        );
    }
    file.write(b"\n</table>");
}