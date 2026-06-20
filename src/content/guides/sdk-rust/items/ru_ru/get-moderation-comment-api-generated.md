## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| comment_id | String | Yes |  |
| include_email | bool | No |  |
| include_ip | bool | No |  |
| sso | String | No |  |

## Ответ

Возвращает: [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_moderation_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment() -> Result<ModerationApiCommentResponse, Error> {
    let params: GetModerationCommentParams = GetModerationCommentParams {
        comment_id: String::from("cmt-48291"),
        include_email: Some(true),
        include_ip: Some(false),
        sso: Some(String::from("sso-acme-corp-2026-token")),
    };
    let response: ModerationApiCommentResponse = get_moderation_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---