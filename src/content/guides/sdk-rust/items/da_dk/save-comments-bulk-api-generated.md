## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_comment_params | Vec<models::CreateCommentParams> | Ja |  |
| is_live | bool | Nej |  |
| do_spam_check | bool | Nej |  |
| send_emails | bool | Nej |  |
| populate_notifications | bool | Nej |  |

## Svar

Returnerer: `Vec<models::SaveCommentsBulkResponse>`

## Eksempel

[inline-code-attrs-start title = 'save_comments_bulk eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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