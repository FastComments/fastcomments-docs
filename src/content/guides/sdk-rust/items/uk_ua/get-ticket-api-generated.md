## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| user_id | String | Ні |  |

## Відповідь

Повертає: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_ticket_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_ticket'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_ticket() -> Result<GetTicket200Response, Error> {
    let params: GetTicketParams = GetTicketParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "support/ticket-2026-03-25-12345".to_string(),
        user_id: Some("user-67890".to_string()),
    };
    let ticket: GetTicket200Response = get_ticket(&configuration, params).await?;
    Ok(ticket)
}
[inline-code-end]

---