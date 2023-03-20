fn main() {
    let d1 = "Rust is a good programming language.";
    let d2 = "Variables are by default immutable in Rust.";
    let d3 = "This is a test text.";
    let d4 = "Rust has a strong static type system.";

    let allDocumentsVec = vec![d1, d2, d3, d4];
    let allDocuments = format!("{}{}{}{}", d1, d2, d3, d4);
    let numberOfDocuments = 4;

    let searchQuery = "Rust";
    // TF Score
    let mut tfScore = (1 + allDocuments.matches(searchQuery).count()) as f32;
    tfScore = tfScore.log2();

    // IDF Score
    let numberOfDocumentsContainingSearchQuery = allDocumentsVec
        .iter()
        .filter(|d| d.contains(searchQuery))
        .count();
    let mut idfScore = numberOfDocuments as f32 / numberOfDocumentsContainingSearchQuery as f32;

    idfScore = idfScore.log2();

    //TF-IDF Score
    let tfIdfScore = tfScore * idfScore;

    println!("{}", tfIdfScore);
}
