use plotpy::Barplot;
use plotpy::Plot;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-29
*/

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FastaStruct {
    id: String,
    seq: String,
}

#[tokio::main]
pub async fn cagplotter(filepath: &str, idinput: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(filepath).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut id: Vec<_> = Vec::new();
    let mut seq: Vec<_> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            id.push(line.clone());
        }
        if !line.starts_with(">") {
            seq.push(line)
        }
    }

    let mut genomevec: Vec<FastaStruct> = Vec::new();
    for i in 0..id.len() {
        genomevec.push(FastaStruct {
            id: id[i].clone().to_string(),
            seq: seq[i].clone().to_string(),
        })
    }

    let mut tokensize: Vec<(String, Vec<&str>)> = Vec::new();

    for i in genomevec.iter() {
        let seqvec: Vec<&str> = i
            .seq
            .as_bytes()
            .windows(3)
            .map(|x| std::str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        let vecinput: (String, Vec<&str>) = (i.id.clone(), seqvec);
        tokensize.push(vecinput);
    }

    let mut genomevec_vec: Vec<(String, usize)> = Vec::new();
    for i in tokensize.iter() {
        let id: String = i.0.clone();
        let seq: Vec<&str> = i.1.clone();
        let mut count: usize = 0usize;
        for i in seq.iter() {
            if *i == "CAG" {
                count += 1
            }
        }
        let unitvector: (String, usize) = (id, count);
        genomevec_vec.push(unitvector);
    }

    let mut plotcag = Vec::new();
    let mut plotcag_count = Vec::new();

    for i in genomevec_vec.iter() {
        if i.0 == idinput {
            plotcag.push(i.0.as_str());
            plotcag_count.push(i.1);
        }
    }

    let mut bar = Barplot::new();
    bar.set_horizontal(true)
        .set_with_text("edge")
        .draw_with_str(plotcag.as_slice(), &plotcag_count);

    let mut plot = Plot::new();
    plot.set_inv_y()
        .add(&bar)
        .set_title("CAG repeat")
        .set_label_x("CAG Count");

    plot.save("cagplot.svg")?;

    Ok("The cag repeats have been analyzed".to_string())
}
