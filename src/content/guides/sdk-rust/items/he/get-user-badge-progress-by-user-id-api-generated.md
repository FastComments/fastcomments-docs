## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| user_id | String | כן |  |

## תגובה

מחזיר: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## דוגמה

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "user-9876".to_string(),
    };
    let _response = get_user_badge_progress_by_user_id(&config, params).await?;
    Ok(())
}
[inline-code-end]

---