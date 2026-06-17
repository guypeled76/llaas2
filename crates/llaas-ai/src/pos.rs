pub fn apply() {
    let model = rust_bert::pipelines::pos_tagging::POSModel::new(Default::default()).unwrap();
    let input = [
        "Are you sure this is working?",
        "本当にこれでうまくいってますか？",
        "Er du sikker på, at dette virker?",
        "¿Estás seguro de que esto funciona?",
    ];
    let output = model.predict(&input);

    for (i, sentence) in output.iter().enumerate() {
        println!("Sentence {}:", i + 1);
        for (word, tag) in sentence.iter().enumerate() {
            println!("{}:{} - {} ({})", word + 1, tag.word, tag.label, tag.score);
        }
    }
}
