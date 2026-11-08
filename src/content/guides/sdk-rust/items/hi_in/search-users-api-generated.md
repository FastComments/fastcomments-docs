## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| url_id | String | हाँ |  |
| username_starts_with | String | नहीं |  |
| mention_group_ids | Vec<String> | नहीं |  |
| sso | String | नहीं |  |
| search_section | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_result.rs)

## उदाहरण

[inline-code-attrs-start title = 'search_users उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_search() -> Result<(), Error> {
    let params = SearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        username_starts_with: Some("john".to_string()),
        mention_group_ids: Some(vec!["group1".to_string(), "group2".to_string()]),
        sso: Some("sso-provider".to_string()),
        search_section: Some("comments".to_string()),
    };
    let _result: SearchUsersResult = search_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]