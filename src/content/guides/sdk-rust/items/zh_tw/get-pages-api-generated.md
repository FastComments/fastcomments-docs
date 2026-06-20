## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |

## 回應

回傳：[`GetPagesApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pages_api_response.rs)

## 範例

[inline-code-attrs-start title = 'get_pages 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetPagesParams = GetPagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        path: Some("news/article".to_string()),
        limit: Some(25),
        cursor: Some("cursor_01AZ".to_string()),
    };
    let pages: GetPagesApiResponse = get_pages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---