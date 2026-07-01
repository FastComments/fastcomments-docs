## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| context_user_id | String | No |  |
| is_live | bool | No |  |

## תגובה

מחזיר: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת delete_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn main() -> Result<(), Error> {
    let params = DeleteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        context_user_id: Some("user-6789".to_string()),
        is_live: Some(true),
    };
    let _result = delete_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]