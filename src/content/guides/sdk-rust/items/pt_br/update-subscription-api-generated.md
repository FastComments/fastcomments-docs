## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| update_api_user_subscription_data | models::UpdateApiUserSubscriptionData | Sim |  |
| user_id | String | Não |  |

## Resposta

Retorna: [`UpdateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_subscription_api_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo update_subscription'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = UpdateSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "sub-12345".to_string(),
        update_api_user_subscription_data: models::UpdateApiUserSubscriptionData {
            plan_id: "premium".to_string(),
            status: "active".to_string(),
        },
        user_id: Some("user-987".to_string()),
    };
    let _resp = update_subscription(config, params).await?;
    Ok(())
}
[inline-code-end]