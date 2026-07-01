## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |

## 回應

返回: [`GetPagesApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pages_api_response.rs)

## 範例

[inline-code-attrs-start title = 'get_pages 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_pages(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
    };
    let _response: GetPagesApiResponse = get_pages(configuration, params).await?;
    Ok(())
}
[inline-code-end]