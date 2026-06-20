頁面上過去的留言者，當前不在線。按 displayName 排序。
在耗盡 /users/online 之後使用此來呈現「Members」區段。
以 commenterName 為游標分頁：伺服器會從 afterName 向前走訪部分索引 {tenantId, urlId, commenterName}，透過 $gt，無 $skip 成本。

## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| after_name | String | 否 |  |
| after_user_id | String | 否 |  |

## 回應

回傳: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## 範例

[inline-code-attrs-start title = 'get_offline_users 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline_users() -> Result<PageUsersOfflineResponse, Error> {
    let params: GetOfflineUsersParams = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/today".to_string(),
        after_name: Some("jane.smith".to_string()),
        after_user_id: Some("user-1024".to_string()),
    };
    let response: PageUsersOfflineResponse = get_offline_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---