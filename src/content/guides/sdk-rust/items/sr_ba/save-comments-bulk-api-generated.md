## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| create_comment_params | Vec<models::CreateCommentParams> | Da |  |
| is_live | bool | Ne |  |
| do_spam_check | bool | Ne |  |
| send_emails | bool | Ne |  |
| populate_notifications | bool | Ne |  |

## Odgovor

Vraća: `Vec<models::SaveCommentsBulkResponse>`

## Primjer

[inline-code-attrs-start title = 'save_comments_bulk Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let comment1: models::CreateCommentParams = models::CreateCommentParams {
        thread_id: "news/article/2026-06-19".to_string(),
        comment_text: "Great reporting — very informative.".to_string(),
        user_id: "user-12345".to_string(),
        parent_id: None,
        mentions: Some(vec![
            models::CommentUserMentionInfo { user_id: "user-678".to_string(), display_name: "Jane Doe".to_string() }
        ]),
        hashtags: Some(vec![
            models::CommentUserHashTagInfo { tag: "analysis".to_string() }
        ]),
        ..Default::default()
    };

    let params: SaveCommentsBulkParams = SaveCommentsBulkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params: vec![comment1],
        is_live: Some(true),
        do_spam_check: Some(true),
        send_emails: Some(false),
        populate_notifications: Some(true),
    };

    let responses: Vec<models::SaveCommentsBulkResponse> = save_comments_bulk(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---