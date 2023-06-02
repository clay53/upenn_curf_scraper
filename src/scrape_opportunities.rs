use std::{path::Path, collections::HashMap, borrow::Borrow};

use html5ever::{ParseOpts, tendril::TendrilSink};
use markup5ever_rcdom::{RcDom, NodeData, Node};

use crate::{load_cookie_string, load_links, Opportunity, CURF_ROOT, Opportunities, Compensation, PreferredStudentYears};

pub async fn scrape_opportunities<P: AsRef<Path>>(path: P, skip_already_pulled: bool) -> tokio::io::Result<()> {
    let cookie_string = load_cookie_string(path.as_ref()).await?;
    let links = load_links(path.as_ref()).await?;

    let client = reqwest::ClientBuilder::new()
        .build().unwrap();
    
    let mut opportunities: HashMap<String, Opportunity> = HashMap::new();
    let links_len_string = links.links_map.len().to_string();
    for (prog, (mut id, _)) in links.links_map.into_iter().enumerate() {
        // id = String::from("metabolic-wiring-epithelial-microbiome-interaction-circuits");

        println!("scraping {:0width$}/{} {:#?}", prog+1, links_len_string, id, width = links_len_string.len());

        let request = client.get(format!("{}/rd/{}", CURF_ROOT, id))
            .header("Cookie", cookie_string.clone());
        
        let result = request.send().await;

        match result {
            Ok(response) => if response.status().is_success() {
                match response.text().await {
                    Ok(document_text) => {
                        let dom = html5ever::parse_document(RcDom::default(), ParseOpts::default())
                            .from_utf8()
                            .read_from(&mut std::io::Cursor::new(document_text.into_bytes()))
                            .unwrap();
                        
                        let main_children =
                            dom
                            .document
                            .children.take()[1]
                            .children.take()[2]
                            .children.take()[3]
                            .children.take()[1]
                            .children.take()[5]
                            .children.take()[3]
                            .children.take()[1]
                            .children.take()[3]
                            .children.take();
                        
                        #[derive(PartialEq)]
                        enum Progress {
                            Start,
                            MentorAreas,
                            Description,
                            PreferredQualifications,
                            Details,
                            PreferredStudentYear,
                            AcademicYear,
                            // PostAcademicYear,
                            // ResearcherName,
                            // ResearcherEmail,
                            // ResearcherLink,
                        }

                        let mut opportunity = Opportunity {
                            mentor_areas: None,
                            description: None,
                            preferred_qualifications: None,
                            preferred_student_years: None,
                            compensation: None,
                            academic_years: None,
                            // researcher_name: None,
                            // researcher_email: None,
                            // researcher_link: None,
                        };
                        
                        let mut progress = Progress::Start;
                        for child in main_children {
                            let children = child.children.take();
                            // if children.len() == 0 && progress != Progress::ResearcherName {
                            if children.len() == 0 {
                                continue;
                            }

                            match progress {
                                Progress::Start => if let NodeData::Text { contents } = &children[0].data {
                                    let contents = contents.borrow().to_string();
                                    match contents.as_str() {
                                        "\n        \n    " => {},
                                        "Mentor Areas" => {
                                            opportunity.mentor_areas = Some(String::new());
                                            progress = Progress::MentorAreas;
                                        },
                                        "Description:" => {
                                            opportunity.description = Some(String::new());
                                            progress = Progress::Description;
                                        }
                                        _ => panic!("unknown header at start {:#?}", contents)
                                    }
                                } else {
                                    panic!("not text in start")
                                },
                                Progress::MentorAreas => {
                                    let mentor_areas_string = opportunity.mentor_areas.as_mut().unwrap();
                                    for data in children.iter().map(|child| &child.data) {
                                        if let NodeData::Text { contents } = data {
                                            let contents = contents.borrow().to_string();
                                            match contents.as_str() {
                                                "Description:" => {
                                                    progress = Progress::Description;
                                                    opportunity.description = Some(String::new());
                                                },
                                                _ => mentor_areas_string.push_str(&format!("{}\n", contents)),
                                            }
                                        }
                                    }
                                },
                                Progress::Description => {
                                    let descripton_string = opportunity.description.as_mut().unwrap();
                                    for data in children.iter().map(|child| &child.data) {
                                        if let NodeData::Text { contents } = data {
                                            let contents = contents.borrow().to_string();                                            match contents.as_str() {
                                                "Preferred Qualifications" => {
                                                    progress = Progress::PreferredQualifications;
                                                    opportunity.preferred_qualifications = Some(String::new());
                                                },
                                                "Details:" => {
                                                    progress = Progress::Details;
                                                },
                                                _ => descripton_string.push_str(&format!("{}\n", contents)),
                                            }
                                        }
                                    }
                                },
                                Progress::PreferredQualifications => {
                                    let preferred_qualifications_string = opportunity.preferred_qualifications.as_mut().unwrap();
                                    for data in children.iter().map(|child| &child.data) {
                                        if let NodeData::Text { contents } = data {
                                            let contents = contents.borrow().to_string();
                                            match contents.as_str() {
                                                "Details:" => {
                                                    progress = Progress::Details;
                                                },
                                                _ => preferred_qualifications_string.push_str(&format!("{}\n", contents)),
                                            }
                                        }
                                    }
                                },
                                Progress::Details => {
                                    if let NodeData::Element { attrs, .. } = &child.data && attrs.borrow().len() > 0 && attrs.borrow()[0].value.to_string() == "row" {
                                        let mut compensation = Compensation {
                                            volunteer: false,
                                            salary: false,
                                            independent_study: false,
                                            work_study: false,
                                        };

                                        compensation.volunteer = if let NodeData::Text { contents } = &children[1].children.take()[3].children.take()[0].data {
                                            contents.borrow().trim() == "Yes"
                                        } else {
                                            panic!("not text in volunteer");
                                        };
                                        compensation.salary = if let NodeData::Text { contents } = &children[3].children.take()[3].children.take()[0].data {
                                            contents.borrow().trim() == "Yes"
                                        } else {
                                            panic!("not text in volunteer");
                                        };
                                        compensation.independent_study = if let NodeData::Text { contents } = &children[5].children.take()[3].children.take()[0].data {
                                            contents.borrow().trim() == "Yes"
                                        } else {
                                            panic!("not text in volunteer");
                                        };
                                        compensation.work_study = if let NodeData::Text { contents } = &children[7].children.take()[3].children.take()[0].data {
                                            contents.borrow().trim() == "Yes"
                                        } else {
                                            panic!("not text in volunteer");
                                        };

                                        opportunity.compensation = Some(compensation);
                                    } else if let NodeData::Text { contents } = &children[0].data {
                                        let contents = contents.borrow().to_string();
                                        match contents.as_str() {
                                            "Preferred Student Year" => {
                                                progress = Progress::PreferredStudentYear;
                                            },
                                            "Academic Year" => progress = Progress::AcademicYear,
                                            "Researcher" => continue,
                                            _ => {
                                                println!("unknown header in details, skipping the rest {:#?}", contents);
                                                continue;
                                            }
                                        }
                                    } else {
                                        panic!("unknown in details")
                                    }
                                },
                                Progress::PreferredStudentYear => if let NodeData::Text { contents } = &children[0].data {
                                    let mut preferred_student_years = PreferredStudentYears {
                                        first_year: false,
                                        second_year: false,
                                        third_year: false,
                                        fourth_year: false,
                                    };
                                    for s in contents.borrow().split(',').map(|s| s.trim()) {
                                        match s {
                                            "First-year" => preferred_student_years.first_year = true,
                                            "Second-Year" => preferred_student_years.second_year = true,
                                            "Junior" => preferred_student_years.third_year = true,
                                            "Senior" => preferred_student_years.fourth_year = true,
                                            _ => panic!("unknown year: {:#?}", s)
                                        }
                                        progress = Progress::Details;
                                    }
                                    opportunity.preferred_student_years = Some(preferred_student_years);
                                } else {
                                    panic!("not text in preferred student year")
                                },
                                Progress::AcademicYear => if let NodeData::Text { contents } = &children[0].data {
                                    let mut academic_years = Vec::new();
                                    for s in contents.borrow().split(',').map(|s| s.trim()) {
                                        academic_years.push(s.parse().unwrap())
                                    }
                                    opportunity.academic_years = Some(academic_years);
                                    // progress = Progress::PostAcademicYear;
                                    break;
                                } else {
                                    panic!("not text in academic year")
                                },
                            }
                        }

                        // println!("{:#?}", opportunity);

                        opportunities.insert(id, opportunity);
                        // panic!();
                    },
                    Err(err) => panic!("{}", err)
                }
            } else {
                println!("Bad status for curf directory: {}, {:#?}", response.status(), response.text().await);
            },
            Err(err) => panic!("{}", err),
        }
    }

    tokio::fs::write(path.as_ref().join("opportunities.ron"), ron::to_string(&Opportunities {
        opportunity_map: opportunities,
    }).unwrap()).await?;

    Ok(())
}