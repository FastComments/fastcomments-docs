## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| skip | i32 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetSsoUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_users_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_sso_users उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetSsoUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10),
    };
    let _response = get_sso_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]