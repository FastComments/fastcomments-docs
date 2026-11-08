## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetPagesApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pages_api_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_pages उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_pages(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
    };
    let _response: GetPagesApiResponse = get_pages(configuration, params).await?;
    Ok(())
}
[inline-code-end]