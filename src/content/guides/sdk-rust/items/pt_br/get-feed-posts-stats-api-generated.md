## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| post_ids | Vec<String> | Sim |  |
| sso | String | Não |  |

## Resposta

Retorna: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/feed_posts_stats_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_feed_posts_stats'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_main(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: GetFeedPostsStatsParams = GetFeedPostsStatsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: vec![
            "news/2026-product-launch".to_string(),
            "blog/engineering-architecture".to_string()
        ],
        sso: Some("sso.jwt.token.eyJhbGci...".to_string()),
    };
    let stats: FeedPostsStatsResponse = get_feed_posts_stats(configuration, params).await?;
    let _api_status: ApiStatus = /* utilize stats conforme necessário */ stats.into();
    Ok(())
}
[inline-code-end]

---