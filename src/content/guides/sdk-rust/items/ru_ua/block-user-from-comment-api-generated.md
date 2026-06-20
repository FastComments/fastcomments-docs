## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| block_from_comment_params | models::BlockFromCommentParams | Да |  |
| user_id | String | Нет |  |
| anon_user_id | String | Нет |  |

## Ответ

Возвращает: [`BlockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_success.rs)

## Пример

[inline-code-attrs-start title = 'Пример block_user_from_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn block_example() -> Result<BlockSuccess, Error> {
    let params: BlockUserFromCommentParams = BlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comments/98765".to_string(),
        block_from_comment_params: models::BlockFromCommentParams {
            reason: "Repeated harassment".to_string(),
            duration_minutes: Some(60 * 24),
            notify_user: Some(true),
        },
        user_id: Some("user_42".to_string()),
        anon_user_id: Some("anon-7a3f".to_string()),
    };
    let success: BlockSuccess = block_user_from_comment(&configuration, params).await?;
    Ok(success)
}
[inline-code-end]

---