## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| create_comment_params | Vec<models::CreateCommentParams> | Oui |  |
| is_live | bool | Non |  |
| do_spam_check | bool | Non |  |
| send_emails | bool | Non |  |
| populate_notifications | bool | Non |  |

## Réponse

Retourne: `Vec<models::SaveComment200Response>`

## Exemple

[inline-code-attrs-start title = 'Exemple de save_comments_bulk'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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