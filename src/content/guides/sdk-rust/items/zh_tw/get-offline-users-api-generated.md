頁面上目前不在線上的過去留言者。按 displayName 排序。
在用完 /users/online 之後使用此項目來渲染「成員」區塊。
基於 commenterName 的游標分頁：伺服器透過 $gt 從 afterName 開始沿著部分索引 {tenantId, urlId, commenterName} 向前走，沒有 $skip 的成本。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| after_name | String | 否 |  |
| after_user_id | String | 否 |  |

## 回應

回傳：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## 範例

[inline-code-attrs-start title = 'get_offline_users 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]