---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| user_id | String | Não |  |

## Resposta

Retorna: [`GetSubscriptionsApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_subscriptions_api_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_subscriptions'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_subscriptions(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSubscriptionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-12345".to_string()),
    };
    let _response: GetSubscriptionsApiResponse = get_subscriptions(config, params).await?;
    Ok(())
}
[inline-code-end]

---