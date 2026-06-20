---
## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odziv

Vrne: [`ApiGetCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comment_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_comment() -> Result<(), Error> {
    let params: GetCommentParams = GetCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-06-19/comment-742".to_string(),
        include_replies: Some(true),
    };
    let comment: ApiGetCommentResponse = get_comment(&configuration, params).await?;
    println!("{:#?}", comment);
    Ok(())
}
[inline-code-end]

---