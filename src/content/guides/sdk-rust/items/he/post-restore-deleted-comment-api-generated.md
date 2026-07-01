## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| comment_id | String | כן |  |
| broadcast_id | String | לא |  |
| sso | String | לא |  |

## תשובה

מחזיר: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## דוגמה

[inline-code-attrs-start title = 'post_restore_deleted_comment דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn restore_comment() -> Result<(), Error> {
    let config: &configuration::Configuration = get_configuration();
    let params = PostRestoreDeletedCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        broadcast_id: Some("broadcast-987".to_string()),
        sso: Some("user@example.com".to_string()),
    };
    let _response = post_restore_deleted_comment(config, params).await?;
    Ok(())
}
[inline-code-end]