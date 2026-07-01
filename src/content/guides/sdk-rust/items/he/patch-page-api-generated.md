## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| update_api_page_data | models::UpdateApiPageData | כן |  |

## Response

מחזיר: [`PatchPageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_page_api_response.rs)

## Example

[inline-code-attrs-start title = 'patch_page דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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