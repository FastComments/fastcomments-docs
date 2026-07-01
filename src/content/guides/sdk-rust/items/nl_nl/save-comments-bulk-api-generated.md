## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_comment_params | Vec<models::CreateCommentParams> | Ja |  |
| is_live | bool | Nee |  |
| do_spam_check | bool | Nee |  |
| send_emails | bool | Nee |  |
| populate_notifications | bool | Nee |  |

## Respons

Retourneert: `Vec<models::SaveCommentsBulkResponse>`

## Voorbeeld

[inline-code-attrs-start title = 'save_comments_bulk Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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