## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_user उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-123".to_string(),
        include_details: Some(true),
    };
    let _response = get_user(configuration, params).await?;
    Ok(())
}
[inline-code-end]