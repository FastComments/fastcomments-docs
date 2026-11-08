## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| url_id | String | हाँ |  |
| title | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `CreateV1PageReact`

## उदाहरण

[inline-code-attrs-start title = 'create_v1_page_react उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateV1PageReactParams = CreateV1PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        title: Some("Rust Community Update".to_string()),
    };
    let _response = create_v1_page_react(&config, params).await?;
    Ok(())
}
[inline-code-end]