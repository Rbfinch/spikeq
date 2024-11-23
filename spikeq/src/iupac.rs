use regex::Regex;

pub fn get_iupac_regexes() -> Vec<(Regex, Vec<&'static str>)> {
    vec![
        (Regex::new(r"\[AG\]").unwrap(), vec!["A", "G"]), // R -> A or G
        (Regex::new(r"\[GA\]").unwrap(), vec!["G", "A"]), // R -> G or A
        (Regex::new(r"\[CT\]").unwrap(), vec!["C", "T"]), // Y -> C or T
        (Regex::new(r"\[TC\]").unwrap(), vec!["T", "C"]), // Y -> T or C
        (Regex::new(r"\[GC\]").unwrap(), vec!["G", "C"]), // S -> G or C
        (Regex::new(r"\[CG\]").unwrap(), vec!["C", "G"]), // S -> C or G
        (Regex::new(r"\[AT\]").unwrap(), vec!["A", "T"]), // W -> A or T
        (Regex::new(r"\[TA\]").unwrap(), vec!["T", "A"]), // W -> T or A
        (Regex::new(r"\[GT\]").unwrap(), vec!["G", "T"]), // K -> G or T
        (Regex::new(r"\[TG\]").unwrap(), vec!["T", "G"]), // K -> T or G
        (Regex::new(r"\[AC\]").unwrap(), vec!["A", "C"]), // M -> A or C
        (Regex::new(r"\[CA\]").unwrap(), vec!["C", "A"]), // M -> C or A
        (Regex::new(r"\[CGT\]").unwrap(), vec!["C", "G", "T"]), // B -> C, G or T
        (Regex::new(r"\[GCT\]").unwrap(), vec!["G", "C", "T"]), // B -> G, C or T
        (Regex::new(r"\[CTG\]").unwrap(), vec!["C", "T", "G"]), // B -> C, T or G
        (Regex::new(r"\[TGC\]").unwrap(), vec!["T", "G", "C"]), // B -> T, G or C
        (Regex::new(r"\[GTC\]").unwrap(), vec!["G", "T", "C"]), // B -> G, T or C
        (Regex::new(r"\[TCG\]").unwrap(), vec!["T", "C", "G"]), // B -> T, C or G
        (Regex::new(r"\[AGT\]").unwrap(), vec!["A", "G", "T"]), // D -> A, G or T
        (Regex::new(r"\[GAT\]").unwrap(), vec!["G", "A", "T"]), // D -> G, A or T
        (Regex::new(r"\[ATG\]").unwrap(), vec!["A", "T", "G"]), // D -> A, T or G
        (Regex::new(r"\[TGA\]").unwrap(), vec!["T", "G", "A"]), // D -> T, G or A
        (Regex::new(r"\[GTA\]").unwrap(), vec!["G", "T", "A"]), // D -> G, T or A
        (Regex::new(r"\[TAG\]").unwrap(), vec!["T", "A", "G"]), // D -> T, A or G
        (Regex::new(r"\[ACT\]").unwrap(), vec!["A", "C", "T"]), // H -> A, C or T
        (Regex::new(r"\[CAT\]").unwrap(), vec!["C", "A", "T"]), // H -> C, A or T
        (Regex::new(r"\[TAC\]").unwrap(), vec!["T", "A", "C"]), // H -> T, A or C
        (Regex::new(r"\[ATC\]").unwrap(), vec!["A", "T", "C"]), // H -> A, T or C
        (Regex::new(r"\[CTA\]").unwrap(), vec!["C", "T", "A"]), // H -> C, T or A
        (Regex::new(r"\[TCA\]").unwrap(), vec!["T", "C", "A"]), // H -> T, C or A
        (Regex::new(r"\[ACG\]").unwrap(), vec!["A", "C", "G"]), // V -> A, C or G
        (Regex::new(r"\[AGC\]").unwrap(), vec!["A", "G", "C"]), // V -> A, G or C
        (Regex::new(r"\[CAG\]").unwrap(), vec!["C", "A", "G"]), // V -> C, A or G
        (Regex::new(r"\[CGA\]").unwrap(), vec!["C", "G", "A"]), // V -> C, G or A
        (Regex::new(r"\[GAC\]").unwrap(), vec!["G", "A", "C"]), // V -> G, A or C
        (Regex::new(r"\[GCA\]").unwrap(), vec!["G", "C", "A"]), // V -> G, C or A
        (Regex::new(r"\[AGCT\]").unwrap(), vec!["A", "G", "C", "T"]), // N -> A, G, C or T
        (Regex::new(r"\[ACGT\]").unwrap(), vec!["A", "C", "G", "T"]), // N -> A, C, G or T
        (Regex::new(r"\[ATCG\]").unwrap(), vec!["A", "T", "C", "G"]), // N -> A, T, C or G
        (Regex::new(r"\[AGTC\]").unwrap(), vec!["A", "G", "T", "C"]), // N -> A, G, T or C
        (Regex::new(r"\[ACTG\]").unwrap(), vec!["A", "C", "T", "G"]), // N -> A, C, T or G
        (Regex::new(r"\[ATGC\]").unwrap(), vec!["A", "T", "G", "C"]), // N -> A, T, G or C
        (Regex::new(r"\[GACT\]").unwrap(), vec!["G", "A", "C", "T"]), // N -> G, A, C or T
        (Regex::new(r"\[GATC\]").unwrap(), vec!["G", "A", "T", "C"]), // N -> G, A, T or C
        (Regex::new(r"\[GCAT\]").unwrap(), vec!["G", "C", "A", "T"]), // N -> G, C, A or T
        (Regex::new(r"\[GCTA\]").unwrap(), vec!["G", "C", "T", "A"]), // N -> G, C, T or A
        (Regex::new(r"\[GTAC\]").unwrap(), vec!["G", "T", "A", "C"]), // N -> G, T, A or C
        (Regex::new(r"\[GTCA\]").unwrap(), vec!["G", "T", "C", "A"]), // N -> G, T, C or A
        (Regex::new(r"\[CAGT\]").unwrap(), vec!["C", "A", "G", "T"]), // N -> C, A, G or T
        (Regex::new(r"\[CATG\]").unwrap(), vec!["C", "A", "T", "G"]), // N -> C, A, T or G
        (Regex::new(r"\[CGAT\]").unwrap(), vec!["C", "G", "A", "T"]), // N -> C, G, A or T
        (Regex::new(r"\[CGTA\]").unwrap(), vec!["C", "G", "T", "A"]), // N -> C, G, T or A
        (Regex::new(r"\[CTAG\]").unwrap(), vec!["C", "T", "A", "G"]), // N -> C, T, A or G
        (Regex::new(r"\[CTGA\]").unwrap(), vec!["C", "T", "G", "A"]), // N -> C, T, G or A
        (Regex::new(r"\[TACG\]").unwrap(), vec!["T", "A", "C", "G"]), // N -> T, A, C or G
        (Regex::new(r"\[TAGC\]").unwrap(), vec!["T", "A", "G", "C"]), // N -> T, A, G or C
        (Regex::new(r"\[TCAG\]").unwrap(), vec!["T", "C", "A", "G"]), // N -> T, C, A or G
        (Regex::new(r"\[TCGA\]").unwrap(), vec!["T", "C", "G", "A"]), // N -> T, C, G or A
        (Regex::new(r"\[TGAC\]").unwrap(), vec!["T", "G", "A", "C"]), // N -> T, G, A or C
        (Regex::new(r"\[TGCA\]").unwrap(), vec!["T", "G", "C", "A"]), // N -> T, G, C or A
    ]
}
