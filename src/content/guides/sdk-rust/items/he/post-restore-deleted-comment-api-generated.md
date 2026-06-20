## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| comment_id | String | כן |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-post_restore_deleted_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = PostRestoreDeletedCommentParams {
        comment_id: String::from("news/article-2024-06-19/comment-8932"),
        sso: Some(String::from("user-session-9f8e7d")),
    };
    let response: ApiEmptyResponse = post_restore_deleted_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---