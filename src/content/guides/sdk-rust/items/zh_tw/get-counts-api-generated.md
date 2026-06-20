## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| sso | String | 否 |  |

## 回應

回傳： [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_count_response.rs)

## 範例

[inline-code-attrs-start title = 'get_counts 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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