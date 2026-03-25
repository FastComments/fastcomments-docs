## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| user_id | String | Hayır |  |
| state | f64 | Hayır |  |
| skip | f64 | Hayır |  |
| limit | f64 | Hayır |  |

## Yanıt

Döndürür: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_tickets Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tickets() -> Result<(), Error> {
    let params: GetTicketsParams = GetTicketsParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        user_id: Some("user-9876".to_owned()),
        state: Some(1.0),
        skip: Some(0.0),
        limit: Some(25.0),
    };
    let tickets: GetTickets200Response = get_tickets(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---