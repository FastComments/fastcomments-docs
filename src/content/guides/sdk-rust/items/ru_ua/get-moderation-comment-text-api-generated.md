## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_moderation_comment_text Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetModerationCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _response: GetCommentTextResponse =
        get_moderation_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]