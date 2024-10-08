use crate::ids::{DatabaseId, PageId, UserId};
use crate::models::properties::{DateOrDateTime, DateValue};
use crate::models::text::{
    Annotations, Link, MentionId, MentionObject, RichText, RichTextCommon, Text, TextColor,
};
use crate::models::users::{Person, User, UserCommon};
use crate::models::{ListResponse, Object, Page};
use std::str::FromStr;
use time::format_description::well_known::Iso8601;
use time::{Date, Month, OffsetDateTime};

#[test]
fn deserialize_page() {
    let _page: Page = serde_json::from_str(include_str!("tests/page.json")).unwrap();
}

#[test]
fn deserialize_query_result() {
    let _page: ListResponse<Page> =
        serde_json::from_str(include_str!("tests/query_result.json")).unwrap();
}

#[test]
fn deserialize_number_format() {
    let _search_results: ListResponse<Object> =
        serde_json::from_str(include_str!("tests/issue_15.json")).unwrap();
}

#[test]
fn rich_text() {
    let rich_text_text: RichText =
        serde_json::from_str(include_str!("tests/rich_text_text.json")).unwrap();
    assert_eq!(
        rich_text_text,
        RichText::Text {
            rich_text: RichTextCommon {
                plain_text: "Rich".to_string(),
                href: Some("https://github.com/jakeswenson/notion".to_string()),
                annotations: Some(Annotations {
                    bold: Some(true),
                    code: Some(true),
                    color: Some(TextColor::Default),
                    italic: Some(true),
                    strikethrough: Some(true),
                    underline: Some(true),
                }),
            },
            text: Text {
                content: "Rich".to_string(),
                link: Some(Link {
                    url: "https://github.com/jakeswenson/notion".to_string()
                }),
            },
        }
    )
}

#[test]
fn rich_text_mention_user_person() {
    let rich_text_mention_user_person: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_user_person.json")).unwrap();
    assert_eq!(
    rich_text_mention_user_person,
    RichText::Mention {
      rich_text: RichTextCommon {
        plain_text: "@John Doe".to_string(),
        href: None,
        annotations: Some(Annotations {
          bold: Some(false),
          code: Some(false),
          color: Some(TextColor::Default),
          italic: Some(false),
          strikethrough: Some(false),
          underline: Some(false),
        }),
      },
      mention: MentionObject::User {
        user: User::Person {
          common: UserCommon {
            id: UserId::from_str("1118608e-35e8-4fa3-aef7-a4ced85ce8e0").unwrap(),
            name: Some("John Doe".to_string()),
            avatar_url: Some(
              "https://secure.notion-static.com/e6a352a8-8381-44d0-a1dc-9ed80e62b53d.jpg"
                .to_string()
            ),
          },
          tag: "person".to_string(),
          person: Person {
            email: "john.doe@gmail.com".to_string()
          },
        }
      }, href: None,
    }
  )
}

#[test]
fn rich_text_mention_date() {
    let rich_text_mention_date: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_date.json")).unwrap();
    assert_eq!(
        rich_text_mention_date,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "2022-04-16 → ".to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            mention: MentionObject::Date {
                date: DateValue {
                    start: DateOrDateTime::Date(
                        Date::from_calendar_date(2022, Month::April, 16).unwrap()
                    ),
                    end: None,
                    time_zone: None,
                }
            },
            href: None,
        }
    )
}

#[test]
fn rich_text_mention_date_with_time() {
    let rich_text_mention_date_with_time: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_date_with_time.json")).unwrap();
    assert_eq!(
        rich_text_mention_date_with_time,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "2022-05-14T09:00:00.000-04:00 → ".to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            mention: MentionObject::Date {
                date: DateValue {
                    start: DateOrDateTime::DateTime(
                        OffsetDateTime::parse("2022-05-14T09:00:00.000-04:00", &Iso8601::DEFAULT)
                            .unwrap()
                    ),
                    end: None,
                    time_zone: None,
                }
            },
            href: None,
        }
    )
}

#[test]
fn rich_text_mention_date_with_end() {
    let rich_text_mention_date_with_end: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_date_with_end.json")).unwrap();
    assert_eq!(
        rich_text_mention_date_with_end,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "2022-05-12 → 2022-05-13".to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            mention: MentionObject::Date {
                date: DateValue {
                    start: DateOrDateTime::Date(
                        Date::from_calendar_date(2022, Month::May, 12).unwrap()
                    ),
                    end: Some(DateOrDateTime::Date(
                        Date::from_calendar_date(2022, Month::May, 13).unwrap()
                    )),
                    time_zone: None,
                }
            },
            href: None,
        }
    )
}

#[test]
fn rich_text_mention_date_with_end_and_time() {
    let rich_text_mention_date_with_end_and_time: RichText = serde_json::from_str(include_str!(
        "tests/rich_text_mention_date_with_end_and_time.json"
    ))
    .unwrap();
    assert_eq!(
        rich_text_mention_date_with_end_and_time,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "2022-04-16T12:00:00.000-04:00 → 2022-04-16T12:00:00.000-04:00"
                    .to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            mention: MentionObject::Date {
                date: DateValue {
                    start: DateOrDateTime::DateTime(
                        OffsetDateTime::parse("2022-04-16T12:00:00.000-04:00", &Iso8601::DEFAULT)
                            .unwrap()
                    ),
                    end: Some(DateOrDateTime::DateTime(
                        OffsetDateTime::parse("2022-04-16T12:00:00.000-04:00", &Iso8601::DEFAULT)
                            .unwrap()
                    )),
                    time_zone: None,
                }
            },
            href: None,
        }
    )
}

#[test]
fn rich_text_mention_page() {
    let rich_text_mention_page: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_page.json")).unwrap();
    assert_eq!(
        rich_text_mention_page,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "Shopping List 2023-01-20 ".to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            mention: MentionObject::Page {
                page: MentionId {
                    id: PageId::from_str("c81ee776-2752-4e98-aa66-c37bd4ba9b8d").unwrap()
                }
            },
            href: Some("https://www.notion.so/c81ee77627524e98aa66c37bd4ba9b8d".to_string())
        }
    );
}

#[test]
fn rich_text_mention_database() {
    let rich_text_mention_database: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_database.json")).unwrap();
    assert_eq!(
        rich_text_mention_database,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "Shopping lists".to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            href: Some("https://www.notion.so/baa9d74593254088a992721dc2fa21dd".to_string()),
            mention: MentionObject::Database {
                database: MentionId {
                    id: DatabaseId::from_str("baa9d745-9325-4088-a992-721dc2fa21dd").unwrap()
                }
            },
        }
    );
}
