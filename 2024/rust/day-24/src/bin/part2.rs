use day_24::part2::process;
use miette::Context;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}

// ! bbb,bch,bvb,bwb,cbh,cpg,cqb,dmk,fcd,fjv,ftc,fvk,ghp,gqg,hct,hnr,kjv,kqm,mfr,msb,mvf,nbk,nmh,nqp,pbb,pww,qdp,qkh,qmn,qqj,qtd,rds,rjp,rqf,sbj,smt,stn,swm,vfv,vmr,wdq,wgw,wtb,wvm,z00,z01,z02,z03,z04,z05,z07,z08,z09,z10,z11,z12,z13,z14,z15,z16,z17,z18,z19,z20,z21,z22,z23,z24,z25,z26,z27,z28,z29,z30,z31,z32,z33,z34,z35,z36,z37,z38,z39,z40,z41,z42,z43,z44

// ! bgc,bhs,bnt,chv,cjf,ckd,cpb,cvm