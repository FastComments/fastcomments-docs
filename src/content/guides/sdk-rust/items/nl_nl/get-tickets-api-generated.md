## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| user_id | String | Nee |  |
| state | f64 | Nee |  |
| skip | f64 | Nee |  |
| limit | f64 | Nee |  |

## Respons

Retourneert: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_tickets Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tickets() -> Result<(), Error> {
    let params: GetTicketsParams = GetTicketsParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("journalist-42")),
        state: Some(1.0),
        skip: Some(0.0),
        limit: Some(50.0),
    };
    let tickets: GetTicketsResponse = get_tickets(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---