//! Domain tag inventory test: verify that all hash domain tags follow the
//! BDLM_ prefix convention and are unique.

use crate::crypto::domain_tags::DOMAIN_TAGS;

#[test]
fn all_domain_tags_have_bdlm_prefix() {
    for tag in DOMAIN_TAGS {
        assert!(
            tag.starts_with("BDLM_") || tag.starts_with("BUDLM_"),
            "domain tag '{}' must start with BDLM_ or BUDLM_",
            tag
        );
    }
}

#[test]
fn domain_tags_are_unique() {
    let mut seen = std::collections::HashSet::new();
    for tag in DOMAIN_TAGS {
        assert!(seen.insert(tag), "duplicate domain tag: {}", tag);
    }
}

#[test]
fn domain_tags_are_ascii() {
    for tag in DOMAIN_TAGS {
        assert!(tag.is_ascii(), "domain tag '{}' is not ASCII", tag);
    }
}

#[test]
fn at_least_ten_domain_tags_exist() {
    assert!(
        DOMAIN_TAGS.len() >= 10,
        "expected at least 10 domain tags, found {}",
        DOMAIN_TAGS.len()
    );
}

#[test]
fn domain_tags_are_nonempty() {
    for tag in DOMAIN_TAGS {
        assert!(!tag.is_empty(), "empty domain tag found");
    }
}

#[test]
fn critical_domain_tags_subset_of_all() {
    use crate::crypto::domain_tags::CRITICAL_DOMAIN_TAGS;
    for tag in CRITICAL_DOMAIN_TAGS {
        assert!(
            DOMAIN_TAGS.contains(tag),
            "critical tag '{}' is not in DOMAIN_TAGS",
            tag
        );
    }
}
