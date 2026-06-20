---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| sso | String | いいえ |  |

## レスポンス

返却値: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_count_response.rs)

## 例

[inline-code-attrs-start title = 'get_counts の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_counts() -> Result<(), Error> {
    let params: GetCountsParams = GetCountsParams {
        sso: Some("acme-corp-tenant".to_string()),
    };
    let counts: GetBannedUsersCountResponse = get_counts(&configuration, params).await?;
    println!("{:?}", counts);
    Ok(())
}
[inline-code-end]

---