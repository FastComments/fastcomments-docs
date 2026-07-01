## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| broadcast_id | String | Да |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример pin_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        broadcast_id: "news/article".to_string(),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = pin_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]