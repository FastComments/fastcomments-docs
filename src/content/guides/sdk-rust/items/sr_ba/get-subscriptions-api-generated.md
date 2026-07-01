## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| user_id | String | Ne |  |

## Odgovor

Vraća: [`GetSubscriptionsApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_subscriptions_api_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_subscriptions Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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