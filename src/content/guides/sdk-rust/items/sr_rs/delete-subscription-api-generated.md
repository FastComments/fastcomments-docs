## Params

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| user_id | String | No |  |

## Odgovor

Vraća: [`DeleteSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_subscription_api_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer delete_subscription'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteSubscriptionParams {
        tenant_id: "acme-corp".to_string(),
        id: "sub-2024-09".to_string(),
        user_id: Some("user-42".to_string()),
    };
    let _response = delete_subscription(&config, params).await?;
    Ok(())
}
[inline-code-end]