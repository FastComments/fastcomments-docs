## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| comment_id | String | Sim |  |
| broadcast_id | String | Sim |  |
| edit_key | String | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_public_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de delete_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteCommentPublicParams = DeleteCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("news/article/2026/03/interesting-story#cmt-67890"),
        broadcast_id: String::from("news-article-12345"),
        edit_key: Some(String::from("editkey-3f2b9a")),
        sso: Some(String::from("sso-jwt-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")),
    };
    let response: DeleteCommentPublic200Response = delete_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---