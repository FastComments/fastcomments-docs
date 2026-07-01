## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| user_id | String | Nie |  |

## Odpowiedź

Zwraca: [`GetSubscriptionsApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_subscriptions_api_response.rs)

## Przykład

[inline-code-attrs-start title = 'get_subscriptions Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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