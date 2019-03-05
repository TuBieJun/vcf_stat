use std::env;
use std::process;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

extern crate regex;
use regex::Regex;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: vcf_stat <input_vcf>");
        process::exit(1);
    }

    let f = File::open(&args[1]).expect(&format!("Can't open {}", &args[1]));
    let reader = BufReader::new(f);
    let mut is_dbsnp:u32 = 0;
    let mut snp_number:u32 = 0;
    let mut indel_number:u32 = 0;
    let mut snp_dbsnp_number = 0;
    let mut indel_dbsnp_number = 0;

    for line in reader.lines(){
        let info = line.unwrap();
        if info.starts_with("#") {
            continue
        }
        let info_vec:Vec<&str> = info.trim().split('\t').collect();
        if ! (info_vec[6] == "PASS" || info_vec[6] == ".") {
            continue
        }
        if info_vec[2] == "." {
            is_dbsnp = 0;
        } else {
            is_dbsnp = 1;
        }
        let format_vec:Vec<&str> = info_vec[9].split(':').collect();
        let re = Regex::new(r"[|/]").unwrap();
        let gt_vec:Vec<&str> = re.split(format_vec[0]).collect();
        let mut allele_list:Vec<&str> = Vec::new();
        allele_list.push(info_vec[3]);
        let alt_list:Vec<&str> = info_vec[4].split('\t').collect();
        allele_list.extend(alt_list);
        //println!("{:?}", gt_vec);
        let index_1:usize = gt_vec[0].parse().unwrap();
        let allele_1 = allele_list[index_1];
        let allele_2:&str;
        if gt_vec.len() == 1 {
            allele_2 = allele_1;
        } else {
            let index_2:usize = gt_vec[1].parse().unwrap();
            allele_2 = allele_list[index_2];
        }
        

        if allele_1.len() == 1 && allele_2.len() == 1 && info_vec[3].len() == 1 {
            snp_number += 1;
            snp_dbsnp_number += is_dbsnp;
        } else {
            indel_number += 1;
            indel_dbsnp_number += is_dbsnp;
        }   

    }
    println!("snp_number\t{}", snp_number);
    println!("snp_dbsnp_ratio\t{:.*}", 2,(snp_dbsnp_number as f32)/(snp_number as f32)  );
    println!("indel_number\t{}", indel_number);
    println!("indel_dbsnp_ratio\t{:.*}", 2,(indel_dbsnp_number as f32) /(indel_number as f32) );
}
