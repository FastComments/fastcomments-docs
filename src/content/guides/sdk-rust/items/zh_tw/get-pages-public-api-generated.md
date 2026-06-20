列出租戶的頁面。由 FChat 桌面客戶端用來填充其聊天室列表。
要求在每個頁面的解析後自訂設定中，`enableFChat` 必須為 true。
需要 SSO 的頁面會根據請求者的群組存取權進行過濾。

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

回傳: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## 範例

[inline-code-attrs-start title = 'get_pages_public 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPagesPublicParams = GetPagesPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    cursor: Some(String::from("cursor_eyJwZl9pZCI6IjEyMyJ9")),
    limit: Some(50),
    q: Some(String::from("tag:release status:published")),
    sort_by: Some(models::PagesSortBy::CreatedAt),
    has_comments: Some(true),
};
let response: GetPublicPagesResponse = get_pages_public(&configuration, params).await?;
[inline-code-end]