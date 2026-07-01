## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| create_feed_post_params | models::CreateFeedPostParams | Sim |  |
| broadcast_id | String | Não |  |
| is_live | bool | Não |  |
| do_spam_check | bool | Não |  |
| skip_dup_check | bool | Não |  |

## Resposta

Retorna: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_posts_response.rs)

## Exemplo

[inline-code-attrs-start title = 'create_feed_post Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".into(),
        create_feed_post_params: models::CreateFeedPostParams {
            text: "Launching new features".into(),
            media: vec![],
        },
        broadcast_id: Some("broadcast-2023-09".into()),
        is_live: Some(true),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let _response = create_feed_post(configuration, params).await?;
    Ok(())
}
[inline-code-end]