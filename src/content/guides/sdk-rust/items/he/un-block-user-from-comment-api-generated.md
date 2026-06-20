## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| un_block_from_comment_params | models::UnBlockFromCommentParams | כן |  |
| user_id | String | לא |  |
| anon_user_id | String | לא |  |

## תגובה

מחזיר: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/unblock_success.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-un_block_user_from_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UnBlockUserFromCommentParams = UnBlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/comments/42".to_string(),
        un_block_from_comment_params: models::UnBlockFromCommentParams {
            reason: Some("mistaken moderation".to_string()),
            unblock_children: Some(true),
        },
        user_id: Some("user-12345".to_string()),
        anon_user_id: None,
    };
    let success: UnblockSuccess = un_block_user_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---