## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| comment_id | String | Yes |  |
| sso | String | No |  |

## תגובה

מחזיר: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_moderation_comment_text'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment_text() -> Result<(), Error> {
    let params: GetModerationCommentTextParams = GetModerationCommentTextParams {
        comment_id: String::from("news/technology/2026/06/19/ai-ethics-12345"),
        sso: Some(String::from("sso-token-7f3a9b2c")),
    };
    let _response: GetCommentTextResponse = get_moderation_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---