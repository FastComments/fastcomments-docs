## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| update_apisso_user_data | models::UpdateApissoUserData | हाँ |  |
| update_comments | bool | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`PatchSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_sso_user_api_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'patch_sso_user उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let update_data = models::UpdateApissoUserData {
        email: Some("john.doe@example.com".to_string()),
        name: Some("John Doe".to_string()),
    };
    let params = PatchSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        update_apisso_user_data: update_data,
        update_comments: Some(true),
    };
    let _response: PatchSsoUserApiResponse = patch_sso_user(&config, params).await?;
    Ok(())
}
[inline-code-end]