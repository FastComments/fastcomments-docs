## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| sso | String | לא |  |

## תגובה

מחזיר: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_count_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_counts'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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