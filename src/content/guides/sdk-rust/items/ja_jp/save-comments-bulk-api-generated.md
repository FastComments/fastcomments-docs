## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_comment_params | Vec<models::CreateCommentParams> | はい |  |
| is_live | bool | いいえ |  |
| do_spam_check | bool | いいえ |  |
| send_emails | bool | いいえ |  |
| populate_notifications | bool | いいえ |  |

## レスポンス

返り値: `Vec<models::SaveCommentsBulkResponse>`

## 例

[inline-code-attrs-start title = 'save_comments_bulk 例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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