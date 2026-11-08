## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ApiModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_moderate_get_user_ban_preferences_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_user_ban_preference उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetUserBanPreferenceParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("user123".to_string()),
    };
    let _response = get_user_ban_preference(configuration, params).await?;
    Ok(())
}
[inline-code-end]