批次取得租戶的使用者資訊。給定 userIds，返回來自 User / SSOUser 的顯示資訊。  
此功能由評論小工具使用，以在使用者透過 presence 事件剛出現時豐富其資訊。  
沒有頁面上下文：隱私會統一強制執行（私人檔案會被遮蔽）。

## Parameters

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| ids | String | 是 |  |

## Response

返回：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Example

[inline-code-attrs-start title = '取得使用者資訊 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]