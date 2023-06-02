use std::{path::Path, io::Write};

use crate::{load_processed_opportunities, CURF_ROOT, ProcessedOpportunity};

pub async fn filter<P: AsRef<Path>>(path: P) -> tokio::io::Result<()> {
    let opportunities = load_processed_opportunities(path).await?;

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    
    let filtered: Vec<(String, ProcessedOpportunity)> = opportunities.processed_opportunity_map.into_iter().filter(|(id, opportunity)| {
        (match &opportunity.academic_years {
            Some(academic_years) => academic_years.contains(&2024) || academic_years.contains(&2023),
            None => true,
        })
        &&
        (match &opportunity.preferred_student_years {
            Some(preferred_student_years) => preferred_student_years.first_year,
            None => true,
        })
        &&
        (match &opportunity.compensation {
            Some(compensation) => compensation.independent_study || compensation.salary || compensation.work_study,
            None => true,
        })
    }).collect();
    let opportunity_count_str = filtered.len().to_string();

    for (index, (id, opportunity)) in filtered.into_iter().enumerate() {
        println!("Title: {}\nlink: {root}/rd/{}", opportunity.title, id, root=CURF_ROOT);
        if let Some(preferred_qualifications) = opportunity.preferred_qualifications {
            println!("Preferred Qualifications: {}", preferred_qualifications.trim());
        }
        loop {
            print!("Open {:0width$}/{}? (y/N{}): ",
                index+1,
                opportunity_count_str,
                if opportunity.description.is_some() || opportunity.mentor_areas.is_some() {
                    "; d: description"
                } else {
                    ""
                },
                width = opportunity_count_str.len()
            );
            stdout.flush().unwrap();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            if input.trim().eq_ignore_ascii_case("y") {
                if let Err(err) = open::that(format!("{}/rd/{}", CURF_ROOT, id)) {
                    println!("Failed to open link: {}",err);
                };
                break;
            } else if input.trim().eq_ignore_ascii_case("d") {
                if let Some(description) = opportunity.description.as_ref() {
                    println!("Description: {}", description.trim());
                }
                if let Some(mentor_areas) = opportunity.mentor_areas.as_ref() {
                    println!("Mentor Areas: {}", mentor_areas.trim())
                }
            } else {
                break;
            }
        }
        println!("~~~~~~~");
    }
    
    println!("Done");

    Ok(())
}