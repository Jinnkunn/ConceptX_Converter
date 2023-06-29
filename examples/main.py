import concept_x_converter

if __name__ == "__main__":
    print(concept_x_converter.version())

    concept_x_converter.processor(
        input="./activation-layer12.json",
        output="output2.txt",
        min=5,
        keep=1,
        seed=1,
    )