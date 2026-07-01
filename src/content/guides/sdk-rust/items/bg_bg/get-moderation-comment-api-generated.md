## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| include_email | bool | No |  |
| include_ip | bool | No |  |
| sso | String | No |  |

## Отговор

Връща: [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за get_moderation_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetModerationCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-6789".to_string(),
        include_email: Some(true),
        include_ip: Some(true),
        sso: Some("sso-user-42".to_string()),
    };
    let _response = get_moderation_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]