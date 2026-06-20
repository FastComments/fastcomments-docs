## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| user_id | String | Не |  |
| state | f64 | Не |  |
| skip | f64 | Не |  |
| limit | f64 | Не |  |

## Одговор

Враћа: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## Пример

[inline-code-attrs-start title = 'get_tickets Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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