## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| create_api_page_data | models::CreateApiPageData | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`AddPageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_page_api_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'add_page उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = AddPageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_page_data: models::CreateApiPageData {
            title: Some("Breaking News".to_string()),
            url: Some("/news/article".to_string()),
            ..Default::default()
        },
    };
    let _response = add_page(&configuration, params).await?;
    Ok(())
}
[inline-code-end]