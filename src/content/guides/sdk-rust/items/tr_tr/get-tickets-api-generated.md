## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| user_id | String | Hayır |  |
| state | f64 | Hayır |  |
| skip | f64 | Hayır |  |
| limit | f64 | Hayır |  |

## Yanıt

Döndürür: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_tickets Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_tickets(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetTicketsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-12345".to_string()),
        state: Some(1.0),
        skip: Some(0.0),
        limit: Some(20.0),
    };
    let _response: GetTicketsResponse = get_tickets(configuration, params).await?;
    Ok(())
}
[inline-code-end]