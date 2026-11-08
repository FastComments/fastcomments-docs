## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| url_id | String | हाँ |  |
| id | String | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: `CreateV1PageReact`

## उदाहरण

[inline-code-attrs-start title = 'delete_v2_page_react उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteV2PageReactParams {
        tenant_id: "acme-corp-tenant".into(),
        url_id: "news/article".into(),
        id: "react-987".into(),
    };
    let _response: CreateV1PageReact = delete_v2_page_react(&config, params).await?;
    Ok(())
}
[inline-code-end]