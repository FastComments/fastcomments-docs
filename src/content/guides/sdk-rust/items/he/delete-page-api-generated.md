## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |

## Response

מחזיר: [`DeletePageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_page_api_response.rs)

## דוגמה

[inline-code-attrs-start title = 'delete_page דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = DeletePageParams {
        tenant_id: "acme-corp-tenant".into(),
        id: "news/article".into(),
    };
    let _resp = delete_page(configuration, params).await?;
    Ok(())
}
[inline-code-end]