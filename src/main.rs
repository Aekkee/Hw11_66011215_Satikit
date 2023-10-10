fn main(){
    
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