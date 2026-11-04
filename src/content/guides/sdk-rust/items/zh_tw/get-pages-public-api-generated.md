列出租戶的頁面。供 FChat 桌面客戶端用於填充其房間列表。
要求每個頁面的已解析自訂設定中 `enableFChat` 為 true。
需要 SSO 的頁面會根據請求使用者的群組存取權限進行篩選。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| cursor | String | 否 |  |
| limit | i32 | 否 |  |
| q | String | 否 |  |
| sort_by | models::PagesSortBy | 否 |  |
| has_comments | bool | 否 |  |

## 回應

回傳:[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## 範例

[inline-code-attrs-start title = 'get_pages_public 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]