## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_comment_params | Vec<models::CreateCommentParams> | Да |  |
| is_live | bool | Нет |  |
| do_spam_check | bool | Нет |  |
| send_emails | bool | Нет |  |
| populate_notifications | bool | Нет |  |

## Ответ

Возвращает: `Vec<models::SaveCommentsBulkResponse>`

## Пример

[inline-code-attrs-start title = 'save_comments_bulk Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params: SaveCommentsBulkParams = SaveCommentsBulkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params: vec![
            models::CreateCommentParams::default(),
            models::CreateCommentParams::default(),
        ],
        is_live: Some(true),
        do_spam_check: Some(false),
        send_emails: Some(true),
        populate_notifications: Some(false),
    };
    let _responses: Vec<models::SaveCommentsBulkResponse> = save_comments_bulk(&configuration, params).await?;
    Ok(())
}
[inline-code-end]