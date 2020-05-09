use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'T' | 'C' | 'G' => {
            dna.chars().try_fold(0, |acc, c| {
                match c {
                    'A' | 'T' | 'C' | 'G' => {
                        if c == nucleotide {
                            Ok(acc + 1)
                        } else {
                            Ok(acc)
                        }
                    },
                    _ => Err(c)
                }
            })

        },
        _ => Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::<char, usize>::new();
    counts.insert('A', 0);
    counts.insert('T', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);

    dna.chars().try_fold(counts, |mut acc, c| {
        match c {
            'A' | 'T' | 'C' | 'G' => {
                let counter = acc.entry(c).or_insert(0);
                *counter += 1;
                Ok(acc)
            },
            _ => Err(c)
        }
    })
}
