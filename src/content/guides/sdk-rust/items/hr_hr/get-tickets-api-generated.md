## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| user_id | String | Ne |  |
| state | f64 | Ne |  |
| skip | f64 | Ne |  |
| limit | f64 | Ne |  |

## Odgovor

Vraća: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer get_tickets'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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