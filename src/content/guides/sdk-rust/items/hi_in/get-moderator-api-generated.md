## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderator_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_moderator उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-123".to_string(),
    };
    let _response: GetModeratorResponse = get_moderator(configuration, params).await?;
    Ok(())
}
[inline-code-end]