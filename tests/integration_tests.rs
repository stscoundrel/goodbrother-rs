use goodbrother::{get_pull_requests_by_user, get_grouped_pull_requests_by_user};
use goodbrother::{PullRequest, Repository};

#[test]
fn gets_pull_requets_by_user() {
    let username = "stscoundrel".to_string();
    let result = get_pull_requests_by_user(username).unwrap();

    assert!(result.len() > 0);

    // Should always contain test PR for Goodbrother.
    let goodbrother_prs: Vec<&PullRequest> = result
        .iter()
        .filter(|pr| pr.repository.eq("stscoundrel/goodbrother"))
        .collect();

    assert!(goodbrother_prs.len() > 0);

    assert!(goodbrother_prs[0].name.eq("Fixture PR for integration tests"));
    assert!(goodbrother_prs[0].link.eq("https://github.com/stscoundrel/goodbrother/pull/18"));
    assert_eq!(goodbrother_prs[0].is_dependabot, false);
}

#[test]
fn gets_grouped_pull_requests_by_user() {
    let username = "stscoundrel".to_string();
    let result = get_grouped_pull_requests_by_user(username).unwrap();

    assert!(result.len() > 0);

    // Should always contain prs for Goodbrother
    let goodbrother_prs: Vec<&Repository> = result
        .iter()
        .filter(|repo| repo.name.eq("stscoundrel/goodbrother"))
        .collect();

    assert!(goodbrother_prs.len() > 0);

    assert!(goodbrother_prs[0].name.eq("stscoundrel/goodbrother"));
    assert!(goodbrother_prs[0].pull_requests[0].name.eq("Fixture PR for integration tests"));
    assert!(goodbrother_prs[0].pull_requests[0].link.eq("https://github.com/stscoundrel/goodbrother/pull/18"));
    assert_eq!(goodbrother_prs[0].pull_requests[0].is_dependabot, false);
}