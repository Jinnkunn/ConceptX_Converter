# ConecptX_Converter
A converter tool for transforming ConceptX activation files into Word2Vec format 

## Usage
Generate a word2vec file from a ConceptX activation file. 
```
cargo run <input_file_name> <output_file_name>
```
Replace <input_file_name> with the name of your ConceptX activation file, and <output_file_name> with the name you would like to give to your Word2Vec file.
<br><br>
For example, if your input file is named my_conceptx_file.txt and you want to name your output file my_word2vec_file.txt, you would run the following command:

```
cargo run text.in.tok.sent_len.activations-layer11.json output.txt
```

The output file will have the following format:

```
<num_words> <dim>
word1:<line_num>:<position_num> val11 val12 val13 ... val1dim
word2:<line_num>:<position_num> val21 val22 val23 ... val2dim
...
```

Where **num_words** is the number of words in the input file, **dim** is the dimension of the word vectors, and **valij** is the jth value of the ith word vector. **line_num** and **position_num** are the line number and position number of the word in the original context (This is designed for the purpose of retrieving the original context of the word from the input file).
<br><br>
Then, in your Python code, you can load the word2vec file using the following code:

```
import gensim

model = gensim.models.KeyedVectors.load_word2vec_format('output.txt', binary=False)

# find the most similar word of the word "He", which is in the 0th line and 0th position of the original context

print(model.most_similar(positive=['He:0:0'], topn=5))
```

## Build Binary
```
cargo build --release
```

## Run Binary
```
./target/release/conceptx_converter <input_file_name> <output_file_name>
```