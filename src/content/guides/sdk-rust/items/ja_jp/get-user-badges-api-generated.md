---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| user_id | String | いいえ |  |
| badge_id | String | いいえ |  |
| displayed_on_comments | bool | いいえ |  |
| limit | f64 | いいえ |  |
| skip | f64 | いいえ |  |

## レスポンス

戻り値: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badges_200_response.rs)

## 例

[inline-code-attrs-start title = 'get_user_badges の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserBadgesParams = GetUserBadgesParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("user-9876")),
        badge_id: Some(String::from("top-reviewer")),
        displayed_on_comments: Some(true),
        limit: Some(50.0),
        skip: Some(0.0),
    };
    let response: GetUserBadges200Response = get_user_badges(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---