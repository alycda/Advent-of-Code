use crate::custom_error::EcError;

// WORDS:RLZ,HRHZ,G,CT,DCC,OW,LXLQ,QXS,ENS,MGUU,VUL,U,IQSZTWLQLJ,SRJITALTAM,DR,CC,NBLR,WXVAPEAUJM,TGSSQZPJNI,E,Y,YEG,CPJ,MJ,XW,PITF,UYUCEJBSRS,DSSG,WOW,OO,WP,UOB,GT,A,DPDVINCYYQ,JW,SXLE,KJWV,PKNW,ZMNJHSVPWW,RBY,IFQF,AXHIDHBYEW,SM,ECWJZXYPLY,EOB,QF,VAGCFZRJUK

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, EcError> {
    let output = input
        .lines()
        .inspect(|line| {
            dbg!(line);
        })
        .collect::<Vec<_>>();

    Ok("0".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // WORDS:THE,OWE,MES,ROD,HER,QAQ
        let input = "AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ";
        assert_eq!("42", process(input)?);
        Ok(())
    }
}
