## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| user_id | String | Nej |  |

## Svar

Returnerer: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_ticket_200_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_ticket Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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