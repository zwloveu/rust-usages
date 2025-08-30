use bills::{Platform, you_know};
use csv::{Reader, ReaderBuilder, StringRecord, Writer, WriterBuilder};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = you_know();

    let input_file: File = File::open("input.csv")?;
    let mut reader: Reader<File> = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input_file);

    // 1. original headers and records
    let headers: StringRecord = reader.headers()?.clone();
    let mut records: Vec<StringRecord> = reader.records().collect::<Result<_, _>>()?;

    // 2. order by repayment date and loan info
    records.sort_by_key(|r: &StringRecord| {
        (
            r.get(1).unwrap_or_default().to_string(),
            r.get(0).unwrap_or_default().to_string(),
        )
    });

    // 3. detailed.csv
    let detailed_file: File = File::create("detailed.csv")?;
    let mut detailed_writer: Writer<File> = WriterBuilder::new().from_writer(detailed_file);
    let mut detailed_headers: Vec<String> = vec!["序号".to_string()];
    detailed_headers.extend(headers.iter().map(|s: &str| s.to_string()));
    detailed_writer.write_record(&detailed_headers)?;
    for (i, record) in records.iter().enumerate() {
        if i != 0 && i % 49 == 0 {
            detailed_writer.write_record(&detailed_headers)?;
        }

        let mut new_record: Vec<String> = record.iter().map(|s: &str| s.to_string()).collect();
        new_record.insert(0, (i + 1).to_string());

        let loan_info: &String = &new_record[1];
        let parts: Vec<&str> = loan_info.split('-').collect();

        if parts.len() >= 1 {
            let original_platform: &str = parts[0];
            let mapped_platform: String = Platform::from_code(original_platform)?
                .chinese_name()
                .to_owned();
            let new_loan_info: String = if parts.len() > 1 {
                format!("{}-{}", mapped_platform, parts[1..].join("-"))
            } else {
                mapped_platform.to_string()
            };
            new_record[1] = new_loan_info;
        }

        detailed_writer.write_record(&new_record)?;
    }
    detailed_writer.flush()?;

    // 4. summary data based on sorted records
    let mut monthly_platform_totals: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut all_platforms: HashSet<String> = HashSet::new();

    for record in records {
        let loan_info: &str = record.get(0).ok_or("缺少借款信息字段")?;
        let original_platform: &str = loan_info.split('-').next().ok_or("借款信息格式错误")?;
        let mapped_platform: String = Platform::from_code(original_platform)?
            .chinese_name()
            .to_owned();
        all_platforms.insert(mapped_platform.to_string());

        let repayment_date: &str = record.get(1).ok_or("缺少还款日期字段")?;
        let month: String = repayment_date
            .get(0..6)
            .ok_or("还款日期格式错误")?
            .to_string();

        let amount_str: &str = record.get(2).ok_or("缺少还款金额字段")?;
        let amount: f64 = amount_str.parse()?;

        let platform_totals: &mut HashMap<String, f64> = monthly_platform_totals
            .entry(month)
            .or_insert_with(HashMap::new);
        *platform_totals
            .entry(mapped_platform.to_string())
            .or_insert(0.0) += amount;
    }

    let mut sorted_months: Vec<String> = monthly_platform_totals.keys().cloned().collect();
    sorted_months.sort();
    let mut sorted_platforms: Vec<String> = all_platforms.into_iter().collect();
    sorted_platforms.sort();

    let summary_file: File = File::create("summary.csv")?;
    let mut writer: Writer<File> = WriterBuilder::new().from_writer(summary_file);

    let mut headers: Vec<String> = vec!["还款月份".to_string()];
    headers.extend(sorted_platforms.iter().cloned());
    headers.push("总还款金额".to_string());
    writer.write_record(&headers)?;

    let mut platform_col_totals: HashMap<String, f64> =
        sorted_platforms.iter().map(|p| (p.clone(), 0.0)).collect();
    let mut grand_total: f64 = 0.0;

    for month in sorted_months {
        let platform_totals: &HashMap<String, f64> = monthly_platform_totals.get(&month).unwrap();
        let mut row: Vec<String> = vec![month.clone()];
        let mut total_amount: f64 = 0.0;

        for platform in &sorted_platforms {
            let amount: f64 = platform_totals.get(platform).copied().unwrap_or(0.0);
            row.push(format!("{:.2}", amount));
            total_amount += amount;

            *platform_col_totals.get_mut(platform).unwrap() += amount;
        }

        row.push(format!("{:.2}", total_amount));
        writer.write_record(&row)?;

        grand_total += total_amount;
    }

    let mut total_row: Vec<String> = vec!["".to_string()];
    for platform in &sorted_platforms {
        total_row.push(format!("{:.2}", platform_col_totals[platform]));
    }
    total_row.push(format!("{:.2}", grand_total));
    writer.write_record(&total_row)?;

    writer.flush()?;
    println!("detail and summary done: detailed.csv, summary.csv");
    Ok(())
}
