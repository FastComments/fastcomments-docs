## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| user_id | String | Non |  |

## Réponse

Renvoie : [`GetTicketResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_ticket_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_ticket'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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