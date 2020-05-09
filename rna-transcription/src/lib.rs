#[derive(Debug, PartialEq)]
pub struct DNA {
    nucs: Vec<char>
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucs: Vec<char>
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let res = dna.chars().enumerate().try_fold(Vec::<char>::new(), |mut acc, (i, c)| {
            match c {
                'A' | 'C' | 'T' | 'G' => {
                    acc.push(c);
                    Ok(acc)
                },
                _ => Err(i)
            }
        });

        match res {
            Ok(x) => Ok(DNA { nucs: x }),
            Err(x) => Err(x)
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA { nucs: self.nucs.iter().map(|c| {
            match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!()
            }
        }).collect()}
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let res = rna.chars().enumerate().try_fold(Vec::<char>::new(), |mut acc, (i, c)| {
            match c {
                'A' | 'C' | 'U' | 'G' => {
                    acc.push(c);
                    Ok(acc)
                },
                _ => Err(i)
            }
        });

        match res {
            Ok(x) => Ok(RNA { nucs: x }),
            Err(x) => Err(x)
        }
    }
}
