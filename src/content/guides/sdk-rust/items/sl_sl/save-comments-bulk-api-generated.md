## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| create_comment_params | Vec<models::CreateCommentParams> | Da |  |
| is_live | bool | Ne |  |
| do_spam_check | bool | Ne |  |
| send_emails | bool | Ne |  |
| populate_notifications | bool | Ne |  |

## Odgovor

Vrne: `Vec<models::SaveCommentsBulkResponse>`

## Primer

[inline-code-attrs-start title = 'save_comments_bulk Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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