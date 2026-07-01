## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| create_feed_post_params | models::CreateFeedPostParams | Sim |  |
| broadcast_id | String | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_response.rs)

## Exemplo

[inline-code-attrs-start title = 'create_feed_post_public Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = CreateFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_feed_post_params: models::CreateFeedPostParams {
            title: "Breaking News".to_string(),
            body: "Details about the news...".to_string(),
            ..Default::default()
        },
        broadcast_id: Some("news/article".to_string()),
        sso: Some("sso-token-abc".to_string()),
    };
    let _response = create_feed_post_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---