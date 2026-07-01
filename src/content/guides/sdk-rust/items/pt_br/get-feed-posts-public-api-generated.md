req  
tenantId  
afterId  

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| after_id | String | Não |  |
| limit | i32 | Não |  |
| tags | Vec<String> | Não |  |
| sso | String | Não |  |
| is_crawler | bool | Não |  |
| include_user_info | bool | Não |  |

## Resposta

Retorna: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_feed_posts_response.rs)

## Exemplo

[inline-code-attrs-start title = 'get_feed_posts_public Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetFeedPostsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post123".to_string()),
        limit: Some(20),
        tags: Some(vec!["news".to_string(), "article".to_string()]),
        sso: Some("sso-token-xyz".to_string()),
        is_crawler: Some(false),
        include_user_info: Some(true),
    };
    let _response = get_feed_posts_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---