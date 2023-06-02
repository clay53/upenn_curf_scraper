use std::{path::Path, collections::HashMap};

use crate::{load_links, load_opportunities, ProcessedOpportunities, ProcessedOpportunity, Opportunity};

pub async fn process_opportunities<P: AsRef<Path>>(path: P) -> tokio::io::Result<()> {
    let links = load_links(path.as_ref()).await?;
    let opportunities = load_opportunities(path.as_ref()).await?;

    let mut processed_opportunities = ProcessedOpportunities {
        processed_opportunity_map: HashMap::new(),
    };

    for (id, link) in links.links_map {
        let opportunity: Opportunity = match opportunities.opportunity_map.get(&id).cloned() {
            Some(opportunity) => opportunity,
            None => continue
        };

        processed_opportunities.processed_opportunity_map.insert(id, ProcessedOpportunity {
            title: link.title,
            short_description: link.description,
            researcher: link.researcher,
            departments: link.departments,
            mentor_areas: opportunity.mentor_areas,
            description: opportunity.description,
            preferred_qualifications: opportunity.preferred_qualifications,
            preferred_student_years: opportunity.preferred_student_years,
            compensation: opportunity.compensation,
            academic_years: opportunity.academic_years,
        });
    }

    tokio::fs::write(path.as_ref().join("processed_opportunities.ron"), ron::to_string(&processed_opportunities).unwrap()).await?;

    Ok(())
}