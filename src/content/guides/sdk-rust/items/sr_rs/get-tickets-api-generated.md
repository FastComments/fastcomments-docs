## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| user_id | String | Не |  |
| state | f64 | Не |  |
| skip | f64 | Не |  |
| limit | f64 | Не |  |

## Одговор

Враћа: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_tickets'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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