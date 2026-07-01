## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|-----------|
| tenant_id | String | Ναι |  |
| create_comment_params | Vec<models::CreateCommentParams> | Ναι |  |
| is_live | bool | Όχι |  |
| do_spam_check | bool | Όχι |  |
| send_emails | bool | Όχι |  |
| populate_notifications | bool | Όχι |  |

## Απόκριση

Επιστρέφει: `Vec<models::SaveCommentsBulkResponse>`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα save_comments_bulk'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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