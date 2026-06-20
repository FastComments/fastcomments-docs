租戶的批次使用者資訊。給定 userIds，返回來自 User / SSOUser 的顯示資訊。
由評論小工具使用，以豐富剛透過 presence 事件出現的使用者。
無頁面上下文：隱私會統一強制執行（私人檔案會被遮蔽）。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## 回應

回傳：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## 範例

[inline-code-attrs-start title = 'get_users_info 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]