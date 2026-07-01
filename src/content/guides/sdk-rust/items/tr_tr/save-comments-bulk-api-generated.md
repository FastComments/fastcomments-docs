## Parametreler

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| create_comment_params | Vec<models::CreateCommentParams> | Evet |  |
| is_live | bool | Hayır |  |
| do_spam_check | bool | Hayır |  |
| send_emails | bool | Hayır |  |
| populate_notifications | bool | Hayır |  |

## Yanıt

Döndürür: `Vec<models::SaveCommentsBulkResponse>`

## Örnek

[inline-code-attrs-start title = 'save_comments_bulk Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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