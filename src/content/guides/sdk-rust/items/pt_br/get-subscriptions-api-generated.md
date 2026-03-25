## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| user_id | String | Não |  |

## Resposta

Retorna: [`GetSubscriptionsApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_subscriptions_api_response.rs)

## Exemplo

[inline-code-attrs-start title = 'get_subscriptions Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetSubscriptionsParams = GetSubscriptionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-42@example.com".to_string()),
    };
    let subscriptions: GetSubscriptionsApiResponse = get_subscriptions(&configuration, params).await?;
    let _ = subscriptions;
    Ok(())
}
[inline-code-end]