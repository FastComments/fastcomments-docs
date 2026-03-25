## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |

## תגובה

מחזיר: [`GetComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetCommentParams = GetCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-12345/comment-6789".to_string(),
    };
    let _response: GetComment200Response = get_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---