## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| badge_id | String | כן |  |
| user_id | String | לא |  |
| comment_id | String | לא |  |
| broadcast_id | String | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/award_user_badge_response.rs)

## דוגמה

[inline-code-attrs-start title = 'put_award_badge דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PutAwardBadgeParams {
        tenant_id: "acme-corp".to_string(),
        badge_id: "top-contributor".to_string(),
        user_id: Some("user-42".to_string()),
        comment_id: Some("comment-99".to_string()),
        broadcast_id: None,
        sso: Some("sso-abc123".to_string()),
    };
    let _response = put_award_badge(configuration, params).await?;
    Ok(())
}
[inline-code-end]