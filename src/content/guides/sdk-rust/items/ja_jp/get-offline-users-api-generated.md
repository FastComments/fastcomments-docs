---
ページ上の過去のコメント投稿者（現在オンラインではない）。displayNameでソート。
/users/online を使い切った後に「Members」セクションを表示するためにこれを使用してください。
commenterName によるカーソルページネーション: サーバーは部分的な {tenantId, urlId, commenterName} を辿ります
afterName から先を $gt でインデックスし、$skip コストは発生しません。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Response

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Example

[inline-code-attrs-start title = 'get_offline_users の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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