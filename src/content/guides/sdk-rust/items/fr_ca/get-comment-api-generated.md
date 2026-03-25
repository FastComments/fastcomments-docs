## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |

## Réponse

Renvoie : [`GetComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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