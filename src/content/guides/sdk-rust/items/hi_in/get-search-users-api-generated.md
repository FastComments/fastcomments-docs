## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| value | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_user_search_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_search_users उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("john.doe".to_string()),
        sso: Some("sso-provider".to_string()),
    };
    let _response: ModerationUserSearchResponse = get_search_users(configuration, params).await?;
    Ok(())
}
[inline-code-end]