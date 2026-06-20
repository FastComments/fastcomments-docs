## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| comment_id | String | כן |  |
| is_flagged | bool | כן |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-flag_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_flag_comment() -> Result<(), Error> {
    let params: FlagCommentPublicParams = FlagCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("comment-89b3"),
        is_flagged: true,
        sso: Some(String::from("sso-uid-7a2f")),
    };

    let _response: ApiEmptyResponse = flag_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---