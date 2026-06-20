## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| sso | String | 아니요 |  |

## 응답

반환: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_count_response.rs)

## 예제

[inline-code-attrs-start title = 'get_counts 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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