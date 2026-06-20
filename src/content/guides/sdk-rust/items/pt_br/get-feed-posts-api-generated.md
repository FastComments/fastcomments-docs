req
tenantId
afterId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| after_id | String | Não |  |
| limit | i32 | Não |  |
| tags | Vec<String> | Não |  |

## Resposta

Retorna: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_response.rs)

## Exemplo

[inline-code-attrs-start title = 'get_feed_posts Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetFeedPostsResponse, Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params: GetFeedPostsParams = GetFeedPostsParams {
        tenant_id: String::from("acme-corp-tenant"),
        after_id: Some(String::from("post_987654321")),
        limit: Some(25),
        tags: Some(vec![String::from("product-updates"), String::from("release")]),
    };
    let response: GetFeedPostsResponse = get_feed_posts(cfg, params).await?;
    Ok(response)
}
[inline-code-end]

---