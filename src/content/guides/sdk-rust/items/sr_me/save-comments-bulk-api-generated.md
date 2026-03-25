## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_comment_params | Vec<models::CreateCommentParams> | Да |  |
| is_live | bool | Не |  |
| do_spam_check | bool | Не |  |
| send_emails | bool | Не |  |
| populate_notifications | bool | Не |  |

## Одговор

Враћа: `Vec<models::SaveComment200Response>`

## Пример

[inline-code-attrs-start title = 'save_comments_bulk Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: SaveCommentsBulkParams = SaveCommentsBulkParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_comment_params: vec![
        models::CreateCommentParams {
            thread_id: "news/article-2026-election".to_string(),
            text: "Insightful reporting — thanks for the coverage!".to_string(),
            author_name: "Jane Doe".to_string(),
            author_email: "jane.doe@example.com".to_string(),
        },
        models::CreateCommentParams {
            thread_id: "news/article-2026-election".to_string(),
            text: "I disagree with the premise of this piece.".to_string(),
            author_name: "John Smith".to_string(),
            author_email: "john.smith@example.org".to_string(),
        },
    ],
    is_live: Some(true),
    do_spam_check: Some(true),
    send_emails: Some(false),
    populate_notifications: Some(true),
};
let saved: Vec<models::SaveComment200Response> = save_comments_bulk(&configuration, params).await?;
[inline-code-end]

---