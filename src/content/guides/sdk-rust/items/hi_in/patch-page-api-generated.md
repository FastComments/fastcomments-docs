## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| update_api_page_data | models::UpdateApiPageData | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`PatchPageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_page_api_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'patch_page उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_patch_page() -> Result<(), Error> {
    let update = models::UpdateApiPageData {
        title: Some("Breaking News".into()),
        content: Some("Updated article content".into()),
        ..Default::default()
    };
    let params = PatchPageParams {
        tenant_id: "acme-corp-tenant".into(),
        id: "news/article".into(),
        update_api_page_data: update,
    };
    let _resp: PatchPageApiResponse = patch_page(&configuration, params).await?;
    Ok(())
}
[inline-code-end]