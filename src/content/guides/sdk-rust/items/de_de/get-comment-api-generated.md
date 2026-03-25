## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Antwort

Gibt zurück: [`GetComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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