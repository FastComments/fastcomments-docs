## Параметри

| Ім’я | Тип | Обов’язково | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| badge_id | String | Так |  |
| user_id | String | Ні |  |
| comment_id | String | Ні |  |
| broadcast_id | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/award_user_badge_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад put_award_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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