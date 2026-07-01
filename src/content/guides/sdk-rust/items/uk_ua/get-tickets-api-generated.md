## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| user_id | String | Ні |  |
| state | f64 | Ні |  |
| skip | f64 | Ні |  |
| limit | f64 | Ні |  |

## Відповідь

Повертає: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_tickets Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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