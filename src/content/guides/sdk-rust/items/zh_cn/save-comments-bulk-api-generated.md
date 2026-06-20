---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| create_comment_params | Vec<models::CreateCommentParams> | 是 |  |
| is_live | bool | 否 |  |
| do_spam_check | bool | 否 |  |
| send_emails | bool | 否 |  |
| populate_notifications | bool | 否 |  |

## 响应

返回: `Vec<models::SaveCommentsBulkResponse>`

## 示例

[inline-code-attrs-start title = 'save_comments_bulk 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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