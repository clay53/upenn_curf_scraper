use std::{path::Path, collections::HashMap, rc::Rc};

use html5ever::{ParseOpts, tendril::TendrilSink};
use markup5ever_rcdom::{RcDom, NodeData, Node};

use crate::{CURF_ROOT, load_cookie_string, Link, Links};

pub async fn scrape_links<P: AsRef<Path>>(path: P) -> tokio::io::Result<()> {
    let cookie_string = load_cookie_string(path.as_ref()).await?;

    let client = reqwest::ClientBuilder::new()
        .build().unwrap();
    
    let mut links: HashMap<String, Link> = HashMap::new();

    let mut page = 0;
    loop {
        println!("scraping page {}", page);
        let request = client.get(format!("{}/undergraduate-research/research-directory?page={}", CURF_ROOT, page))
            .header("Cookie", cookie_string.clone());
        // println!("{:#?}", request);
        let result = request.send().await;
        match result {
            Ok(response) => if response.status().is_success() {
                match response.text().await {
                    Ok(document_text) => {
                        let dom = html5ever::parse_document(RcDom::default(), ParseOpts::default())
                            .from_utf8()
                            .read_from(&mut std::io::Cursor::new(document_text.into_bytes()))
                            .unwrap();
                        let cards_container =
                            dom.
                            document
                            .children.take()[1]
                            .children.take()[2]
                            .children.take()[3]
                            .children.take()[1]
                            .children.take()[5]
                            .children.take()[3]
                            .children.take()[1]
                            .children.take()[3]
                            .children.take()[5]
                            .children.take()[1]
                            .children.take()[1]
                            .children.take()[1]
                            .children.take()[0].children.take();
                        let cards: Vec<&Rc<Node>> = (3..=cards_container.len()-4).step_by(2).map(|i| &cards_container[i]).collect();
                        for card in cards {
                            let main_children =
                                card.children.take()[0]
                                .children.take()[1]
                                .children.take();
                            let title_element = &main_children[1].children.take()[0];
                            let id = if let NodeData::Element { attrs, .. } = &title_element.data {
                                attrs.borrow()[0].value[4..].to_string()
                            } else {
                                panic!("not element for title element")
                            };
                            let title = if let NodeData::Text { contents } = &title_element.children.take()[0].data {
                                (*contents).borrow().to_string()
                            } else {
                                panic!("not text for title");
                            };
                            let description = if let NodeData::Text { contents } = &main_children[3].children.take()[0].data {
                                (*contents).borrow().to_string()
                            } else {
                                panic!("not text for description");
                            };
                            let info_children = main_children[5].children.take();
                            let (researcher, departments) = if info_children.len() == 7 {
                                (
                                    Some(if let NodeData::Text { contents } = &info_children[1].children.take()[0].data {
                                        (*contents).borrow().to_string()
                                    } else {
                                        panic!("not text for researcher")
                                    }),
                                    {
                                        let mut departments = Vec::new();
                                        let departments_children = info_children[5].children.take();
                                        for i in (1..departments_children.len()).step_by(2) {
                                            if let NodeData::Text { contents } = &departments_children[i].children.take()[0].data {
                                                departments.push((*contents).borrow().trim_end_matches(',').to_string());
                                            } else {
                                                panic!("not text for description")
                                            }
                                        }
                                        departments
                                    }
                                )
                            } else if info_children.len() == 5 {
                                (
                                    None,
                                    {
                                        let mut departments = Vec::new();
                                        let departments_children = info_children[3].children.take();
                                        for i in (1..departments_children.len()).step_by(2) {
                                            if let NodeData::Text { contents } = &departments_children[i].children.take()[0].data {
                                                departments.push((*contents).borrow().trim_end_matches(',').to_string());
                                            } else {
                                                panic!("not text for description")
                                            }
                                        }
                                        departments
                                    }
                                )
                            } else {
                                panic!("unknown info for card")
                            };
                            links.insert(id, Link {
                                title,
                                description,
                                researcher,
                                departments,
                            });
                        }

                        let navigation_items = cards_container[cards_container.len()-2].children.take()[3].children.take();
                        if let NodeData::Element { attrs, .. } = &navigation_items[navigation_items.len()-4].data {
                            if &attrs.borrow()[0].value.to_string() == "page-item pager__item pager__item--next" {
                                page += 1;
                            } else {
                                println!("finished on {}", page);
                                break;
                            }
                        } else {
                            panic!("navigation not element");
                        }
                    },
                    Err(err) => panic!("{}", err)
                }
            } else {
                println!("Bad status for curf directory: {}, {:#?}", response.status(), response.text().await);
            },
            Err(err) => panic!("{}", err),
        }
    }
    tokio::fs::write(path.as_ref().join("links.ron"), ron::to_string(&Links {
        links_map: links,
    }).unwrap()).await?;
    Ok(())
}