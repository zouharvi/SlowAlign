---
title: "SlowAlign Report"
author: |
        | Vilém Zouhar
        | <zouhar@ufal.mff.cuni.cz>
date: February 2021
geometry: margin=2.5cm
output: pdf_document
---


The main functionalities of SlowAlign are (1) heuristic parameter estimation in a supervised fashion using gridsearch, (2) combination of multiple soft alignments and (3) data-less alignment based on diagonal alignment, levenstein distance and bluring. The project is hosted at [github.com/zouharvi/SlowAlign](https://github.com/zouharvi/SlowAlign/).

This report is split into the following sections:

- [System Description](#System-Description): introduction to different components of SlowAlign together with the heuristics
- [Evaluation](#Evaluation): dataset overview and evaluation, esp. in comparison to `fast_align`
- [Future Work](#Future-Work): list of further improvements
- [Appendix A](#Appendix-A): technical details including building this project

# System Description

The system is currently composed of two parts: [SlowAlign-Dic](##SlowAlign-Dic) and [SlowAlign-Main](##SlowAlign-Main).

## SlowAlign Dic

A viable way to align one sentence pair `(test_s0, test_t0)` given that we already have large parallel corpus `{(train_s0, train_t0), (train_s1, train_t1), ..)}` is to simply add the test sentence pair to the training data and run the unsupervised alignment algorithm again:

```
test_s0  ||| test_t0
train_s0 ||| train_t0
train_s1 ||| train_t1
...
```

This however does not scale well, especially if the user is interested in online usage of the alignment (data come in sequentially and not in batches). To alleviate this, it is possible to simply store the word translation probabilities (output of the Expectation step in IBM1) and load them at inference time. The results will not be similar as in the first case (the presence of the test sentence can influence the outcome of the training algorithm), but the changes may be considered negligible and in general this makes it possible to deploy and expect reasonable runtime. The tool `fast_align` also has this feature, though undocummented.

Another advantage is that one can re-use pretrained word translation probabilities of other systems. Namely [OPUS](https://opus.nlpl.eu/) provides a great variety of such pre-trained word translation dictionaries (_dic_). The second column is the translation probability, the third is the source token and the last column is the target token. The first column is the number of occurences together. An example from Ubuntu v14.10 (de -> en), further columns omitted:

```
...
2	0.117647058823529	Abmeldeknopf     logout	  
5	0.192307692307692	Abmelden         Log Out	  
7	0.181818181818182	Abmelden         logout	  
3	0.113207547169811	Abmelden         Log out	  
5	0.0980392156862745	Abmelden         logging	  
2	0.0727272727272727	Abmelden         Logout	  
2	0.114285714285714	Abmeldeoption    logout	  
4	0.363636363636364	Abmessungen      Dimensions
5	0.27027027027027	Abmessungen      dimensions
...
```

Storing the whole translation matrix would lead to |V|x|V| number of entries, which is undesirable, especially for word pairs with translation probability close to 0. We therefore need to decide a threshold by which we decide if a given pair of words is to be stored or not.

SlowAlign-dic simply takes in two files of sentences to be aligned, the mentioned threshold and outputs the word translation dictionary (first column contains a dummy value, as it is not used). For word translation proability estimation, IBM1 model without NULL tokens is used. See [Appendix A](#Appendix-A) for further usage information of SlowAlign-dic (binary `slow_align_dic`).

## SlowAlign Main

The output of this component is always an alignment (0-indexed) of the input (either files or sentences passed through the CLI). Additional tasks may be performed, such as searching for optimal parameters or evaluating the performance when gold alignments are supplied. The stdout is always reserved for just the alignment. See [Appendix A](#Appendix-A) for detailed usage information of SlowAlign (binary `slow_align`).

### Extractors

Traditionally, word alignment consists of two parts: (1) soft alignment, which produces alignment scores and (2) induction of hard alignment, which produces the alignment itself from the scores. SlowAlign is espetially targeted to improve on the hard alignment. The soft alignment can be the intermediate representation in IBM models, but it can also be other metrics: attention energies, difference in word position in the sentence, levenstein distance, etc. 

$\mathbf{A_1}$: In IBM Model 1, the hard alignment induction is done by an argmax from the target side: align every target token with the source token of the mutual higher alignment score. This makes a strong assumption that every target token is aligned to exactly one source token. Because of this, the IBM Model 1 also uses NULL tokens, so that a target word has the possibility to align itself to NULL.

$\mathbf{A_2^\alpha}$: Even simpler approach is to consider every alignment with the score above some threshold. This makes this extractor parametrized by one value $\alpha$. In essence, the expressive power of this is higher than that of $A_1$, because it does not impose any restrictions on the number of alignments. This is at the cost of not considering the context of possible values for every target token.

$\mathbf{A_3^\alpha}$: The threshold can also be set dynamically. For every source token $s$, compute the threshold as $\alpha \times max_t\{\text{score}(s,t)\}$ and  then take all alignments with score at least thsi value. The alpha values are bounded between $0$ - take everything and $1$ - take only the argmax (+every alignment with score equal to the maximum). Note: this can be further generalized to accomodate also negative scores by dividing instead of multiplying.

$\mathbf{A_4^\alpha}$: The last extractor is equivalent to $\mathbf{A_3^\alpha}$ with the only difference of aggregating from the target side.

**Combination**: Further improvements can be made by the combination of the two via set operations. Usually the union improves the recall, while the precision is improved by the intersection. For example, the semantics of $A_1 \cap A_2^\alpha$ would be: _align every target token to its maximum source counterpart, but with the rule of all scores begin above $\alpha$_. Finally, an extra improvement is to consider alignment from both directions. Note that for example the output of IBM Model 1 is not just the transpose of the output on switched parallel data, because the E-M algorithm does asymetric operations.

The main formula used in SlowAlign is parametrized by 7 real values:

\begin{align*}
& \mathbf{A_5^\alpha} = \\
& \Big[
\big[A_4^{\alpha_1}(\text{IBM}_\text{fwd}) \cap A_3^{\alpha_2}(\text{IBM}_\text{rev}) \cap A_2^{\alpha_3}(\text{diag}) \cap A_2^{\alpha_4}(\text{blur}^{\alpha_5}(\text{IBM}_\text{fwd})) \big]
 \cup A_2^{\alpha_6}(\text{levenstein})
\Big]
\cap A_4^{\alpha_7}(\text{IBM}_\text{fwd})
\end{align*}

The soft alignments are the following:

- $\text{IBM}_\text{fwd}$: IBM Model 1 without the NULL tokens.
- $\text{IBM}_\text{rev}$: IBM Model 1 without the NULL tokens performed on switched data (target-source) and then transposed.
- diag: Absolute value of relative positions in the sentence: $\big| \frac{i}{|S|} - \frac{j}{|T|}\big|$
- levenstein: Levenstein distance of two words. This is useful for non-text tokens, such as interpunction, but can be also used for e.g. the alignment of post-edited  dialect to standard language.
- blur: Applies blurring filter $[0, \alpha, 0], [\alpha, 1-4*\alpha, \alpha], [0, \alpha, 0]$ on inside nodes of the soft alignment. This is motivated by the fact, that if adjacent words have high scores to be aligned to the same target word, then the source word in the middle is also probably aligned to the same target word.  

The parameters are specified in this format (square brackets mandatory): $[\alpha_1],[\alpha_2],[\alpha_3],[\alpha_4,\alpha_5],[\alpha_6],[\alpha_7]$. Default is `[0.0],[1.0],[0.8],[0.0,0.1],[0.95],[0.8]`.

The gridsearch method searches the parameters in the following space. The behaviour of `linspace` is similar to the [one of NumPy](https://numpy.org/doc/stable/reference/generated/numpy.linspace.html) (endpoint included). This space is defined in `src/optimizer.rs`.  

```
linspace(0.95, 1.0, 4),
linspace(0.90, 1.0, 6),
linspace(0.1, 1.0, 10),
cartesian_product([linspace(0.1, 0.3, 8), linspace(0.0, 0.005, 4)]),
linspace(0.7, 1.0, 4),
linspace(0.0, 0.005, 4),
```

\newpage

### Methods

The following table lists methods available under the argument `--method` together with a minimum description. The methods were chosen to fill a specific requirement niche. Their top-level behavior is defined in `src/main.rs`.

\renewcommand\arraystretch{1.5}  
|Name|Comment|Extraction|Purpose|
|:-|:-----|:--|:---|
|`ibm1`|Standard IBM1 model (without NULL tokens).| argmax ($\mathbf{A_1})$|Baseline comparison to other methods.|
|`levenstein`|Alignment score of two words is based on their levenstein distance: $1.0 - \frac{\text{lev}(s,t)}{|s|+|t|}$|threshold ($\mathbf{A_2}$)\newline default `[0.75]`|Lexical based approximation of alignment.|
|`static`|Combination of diagonal alignment and levenstein. Soft alignment is the arithmetic average of levenstein distance and $\big|\frac{i}{|S|}-\frac{j}{|T|} \big|$. Default method of `slow_align`.|threshold ($\mathbf{A_2}$)\newline default `[0.5]`|Alignment of dialect and standardized language for which little data is available. (Used by a classmate.)|
|`search`|Performs gridsearch over hardcoded subset of the possible values of $\alpha$. The (first) set of parameters with the lowest AER is then outputted. Parameters `--dev-count` controls how many sentences (from the top) are used for parametric estimation. For final evaluation, `--test-offset` determines from which sentence (until the end of supplied aligned sentences) to compute the final AER.|Searched space above. Defined in `src/optimizer.rs`.|Using small number of supervised examples to achieve better performance.|
|`dic`|A combination of multiple soft alignments. Requires OPUS-like translation probability table passed by `--dic`. This table can be either downloaded from OPUS or trained using `slow_align_dic`.|$\mathbf{A_5}$ (formula above)\newline default above|Fast inference given the parameters of `search`.|
\renewcommand\arraystretch{1}  


Parameters have defaults which can be changed using the `--params` argument. Currently there is no mechanism nor typing system to enforce shape of the parameters. The structure is however invariant for every method and it can be observed from the defaults. Extra parameters will be ignored; not enough parameters will cause a panic.

# Evaluation

TODO

## Datasets

TODO

## Train Data Size

TODO

# Future Work

**Built-in http server:** Word alignment can be used in multiple scenarios. It can be useful for research purposes as a binary installed on a computer, but it is also necessary in some deployment scenarios. Furthermore, sharing an alignment service in a collaboration is easier than to manually send around parallel data to align or to learn to use a specific word alignment tool.
The simplest solution would just accept the request and call the appropriate method from the main binary. This would be resource wasteful, since e.g. the word translation dictionary would have to be loaded from the filesystem (or cached in the memory by the OS) and parsed for every request (usually a pair of sentences). A solution to this would be to specify list of dictionaries to load when starting the server (more secure, but also more restrictive) or keep explicit cache of one most recently used dictionary. 

**Gridsearch multithreading:** Currently, only the IBM soft alignment implementation is multithreaded (fixed to 4 threads), even though it is problematic and bottlenecked between the Expectation and Maximization steps. Inducing hard alignment using the recipes, is however a pure function which only needs a read access to  the soft alignment package. At the end, only the argmax needs to be extracted (parameters) together with the AER. Even though this does not influence all the use-cases and especially the inference scenario, it is possible to gain a multiplicative speedup determined solely by the number of cores (e.g. 8x). This is also true for the levenstein computation.

**Custom alignment score input:** It has been shown, that attention scores can be used for word alignment, especially if one of the attention heads is trained for this explicitly. Further improvements can be made if these scores are joined with other soft alignments. Main advantage is also the more complex hard alignment induction scheme. An MT practicioner who wishes to e.g. also present the word alignments next to the MT output may choose to send a request to SlowAlign together with their attention scores (and possibly other parameters) to get a better hard alignment.

# Appendix A

SlowAlign is written in Rust (1.50 in this report). For installation, please refer to the [official resources](https://www.rust-lang.org/tools/install). Assuming that `cargo` is in the path, the main binary can be run as: `cargo run --release --bin slow_align -- <arguments>`. The target binary is going to be stored in `target/release/slow_align`. To just build the binary and not run it, replace `run` with `build`. 

Output of `slow_align --help`:

```
USAGE:
    slow_align [FLAGS] [OPTIONS]

FLAGS:
        --gold-one-index
            Treat gold alignments as if they are 1-indexed (default is 0-indexed)

    -h, --help
            Prints help information

        --lowercase
            Treat everything case-insensitive (default is case-sensitive, even though that
            provides slightly worse performance).

    -V, --version
            Prints version information


OPTIONS:
    -d, --dic <dic>
            OPUS-like dictionary of word translation probabilities

    -f, --file1 <file1>
            Path to the source file to align. (If both files and sentences are provided,
            only files are used).

    -f, --file2 <file2>
            Path to the target file to align. (If both files and sentences are provided,
            only files are used).

    -g, --gold <gold>
            Path to the file with alignments (single space separated, x-y for sure alignments,
            x?y for possible). `x` and `y` are (by default) 0-indexed token indicies.

    -m, --method <method>
            Which alignment method pipeline to use (static, dic, levenstein, ibm1, search)
            [default: static]

    -p, --params <params>
            Comma-separated arrays of parameters to the estimator recipe
            [default: [0.0],[0.0],[1.0],[0.8],[0.0,0.1],[0.95],[0.8]]

    -s, --sent1 <sent1>
            List of source sentences (separated by \n) to align.

    -s, --sent2 <sent2>
            List of target sentences (separated by \n) to align.
```

&nbsp;

Output of `slow_align_dic --help`:

```
USAGE:
    slow_align_dic [FLAGS] [OPTIONS] <file1> <file2> <out>

ARGS:
    <file1>
            Path to the source file to train word translation probabilities on.
    <file2>
            Path to the target file to train word translation probabilities on.
    <out>
            Path to the output word translation probabilities dictionary.

FLAGS:
    -h, --help
            Prints help information

        --lowercase
            Treat everything case-insensitive (default is case-sensitive, even though
            that provides slightly worse performance).

    -V, --version
            Prints version information


OPTIONS:
    -t, --threshold <threshold>
            Threshold under which translation probabilities will be omitted. Lower values
            lead to better approximation, but also larger file size. [default: 0.2]
```