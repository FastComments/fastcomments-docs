## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| comment_id | String | כן |  |
| broadcast_id | String | כן |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-pin_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_pin() -> Result<ChangeCommentPinStatusResponse, Error> {
    let params: PinCommentParams = PinCommentParams {
        tenant_id: "acme-news".to_string(),
        comment_id: "cmt-9f8b7d6".to_string(),
        broadcast_id: "news/article/2026/06/19/article-12345".to_string(),
        sso: Some("user-ssotoken-abc123".to_string()),
    };
    let response: ChangeCommentPinStatusResponse = pin_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---