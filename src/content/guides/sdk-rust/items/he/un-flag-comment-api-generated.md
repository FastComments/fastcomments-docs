## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| user_id | String | לא |  |
| anon_user_id | String | לא |  |

## תגובה

מחזיר: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-un_flag_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn unflag_example() -> Result<FlagCommentResponse, Error> {
    let params: UnFlagCommentParams = UnFlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-98765".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let response: FlagCommentResponse = un_flag_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---