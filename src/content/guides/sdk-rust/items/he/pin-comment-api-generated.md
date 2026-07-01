## פרמטרים

| שם | סוג | הכרחית | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| comment_id | String | כן |  |
| broadcast_id | String | כן |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של pin_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        broadcast_id: "news/article".to_string(),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = pin_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---