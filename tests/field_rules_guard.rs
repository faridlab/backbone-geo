//! The generated write guard checks a country's field attributes: a blank required field
//! refuses the write, a length limit runs in shadow, and an update is checked only for the
//! fields it changes.

use backbone_core::write_guard::{WriteCtx, WriteGuard, WriteKind};
use backbone_geo::application::service::CountryRowRules;
use backbone_geo::domain::entity::Country;

fn codes(v: &[backbone_core::violation::Violation]) -> Vec<(String, String)> {
    v.iter().map(|x| (x.path.clone(), x.code.clone())).collect()
}

#[tokio::test]
async fn a_blank_required_field_refuses_a_create() {
    let country = Country::new("".into(), "Indonesia".into(), 62);
    let out = CountryRowRules.check(&WriteCtx::new(WriteKind::Create, None, Some(&country))).await;
    assert_eq!(codes(&out.refuse), vec![("isocode".to_string(), "required".to_string())]);
}

#[tokio::test]
async fn a_length_limit_runs_in_shadow() {
    let country = Country::new("IDN".into(), "x".repeat(200), 62);
    let out = CountryRowRules.check(&WriteCtx::new(WriteKind::Create, None, Some(&country))).await;
    assert!(out.refuse.is_empty(), "{:?}", codes(&out.refuse));
    assert_eq!(codes(&out.shadow), vec![("name".to_string(), "max_length".to_string())]);
    assert_eq!(out.shadow[0].params.get("max"), Some(&serde_json::json!(120)));
}

#[tokio::test]
async fn an_update_is_checked_only_for_the_fields_it_changes() {
    let before = Country::new("".into(), "Indonesia".into(), 62);
    let mut after = before.clone();
    after.phonecode = 63;
    let out = CountryRowRules.check(&WriteCtx::new(WriteKind::Update, Some(&before), Some(&after))).await;
    assert!(out.refuse.is_empty(), "an untouched blank field does not refuse an edit: {:?}", codes(&out.refuse));
    let mut blanked = before.clone();
    blanked.isocode = "IDN".into();
    let mut emptied = blanked.clone();
    emptied.isocode = "  ".into();
    let out = CountryRowRules.check(&WriteCtx::new(WriteKind::Update, Some(&blanked), Some(&emptied))).await;
    assert_eq!(codes(&out.refuse), vec![("isocode".to_string(), "required".to_string())]);
}
