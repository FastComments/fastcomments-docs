---
現在オンラインのページ閲覧者: 現在そのページにサブスクライブされている WebSocket セッションを持つユーザーです。
返却は anonCount と totalCount の合計です（部屋全体の購読者で、列挙しない匿名閲覧者も含みます）。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| url_id | String | はい |  |
| after_name | String | いいえ |  |
| after_user_id | String | いいえ |  |

## レスポンス

返却: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## 例

[inline-code-attrs-start title = 'get_online_users の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("john_doe".to_string()),
        after_user_id: Some("user-123".to_string()),
    };
    let _response: PageUsersOnlineResponse = get_online_users(&config, params).await?;
    Ok(())
}
[inline-code-end]

---