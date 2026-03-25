---
## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| create_comment_params | Vec<models::CreateCommentParams> | Sí |  |
| is_live | bool | No |  |
| do_spam_check | bool | No |  |
| send_emails | bool | No |  |
| populate_notifications | bool | No |  |

## Respuesta

Devuelve: `Vec<models::SaveComment200Response>`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de save_comments_bulk'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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