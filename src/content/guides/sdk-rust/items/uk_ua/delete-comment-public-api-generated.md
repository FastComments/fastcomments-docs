## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| broadcast_id | String | Так |  |
| edit_key | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`PublicApiDeleteCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_delete_comment_response.rs)

## Приклад

[inline-code-attrs-start title = 'delete_comment_public Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteCommentPublicParams = DeleteCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("cmt-7f3a2b9"),
        broadcast_id: String::from("news/article/2026/06/19/article-12345"),
        edit_key: Some(String::from("editkey-9d2f")),
        sso: Some(String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")),
    };
    let response: PublicApiDeleteCommentResponse = delete_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---