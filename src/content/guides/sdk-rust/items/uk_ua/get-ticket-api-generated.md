## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| user_id | String | Ні |  |

## Відповідь

Повертає: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_ticket_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_ticket'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_ticket() -> Result<GetTicketResponse, Error> {
    let params: GetTicketParams = GetTicketParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "TICKET-2026-045".to_string(),
        user_id: Some("user-12345".to_string()),
    };
    let ticket: GetTicketResponse = get_ticket(&configuration, params).await?;
    Ok(ticket)
}
[inline-code-end]

---