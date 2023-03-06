use shfonts::{Location, UrlByLine, UrlData};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn setup(list: &mut Vec<UrlData>) -> UrlByLine {
    list.push(UrlData {
        url: "https://some.url/font.tff".to_string(),
        location: Location {
            line: 4,
            column: 20,
        },
    });
    list.push(UrlData {
        url: "https://some.url/font2.tff".to_string(),
        location: Location {
            line: 8,
            column: 20,
        },
    });
    list.push(UrlData {
        url: "https://some.url/font3.tff".to_string(),
        location: Location {
            line: 8,
            column: 40,
        },
    });
    UrlByLine::new(list)
}

#[test]
fn ubl_has_list() -> TestResult {
    let mut list: Vec<UrlData> = Vec::new();
    let ubl = setup(&mut list);
    assert_eq!(ubl.len(), 3);
    Ok(())
}

#[test]
fn ubl_at_returns_none_when_line_number_does_not_match() -> TestResult {
    let mut list: Vec<UrlData> = Vec::new();
    let ubl = setup(&mut list);
    assert_eq!(None, ubl.at(0));
    Ok(())
}

#[test]
fn ubl_at_returns_some_with_matching_line_number() -> TestResult {
    let mut list: Vec<UrlData> = Vec::new();
    let ubl = setup(&mut list);
    let data = ubl.at(4).unwrap();
    let datum = &data[0];
    assert_eq!("https://some.url/font.tff".to_string(), datum.url);
    Ok(())
}
