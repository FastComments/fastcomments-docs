## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| user_id | String | No |  |

## Odgovor

Vraća: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_ticket_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_ticket'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetTicketParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "ticket-456".to_string(),
        user_id: Some("user-123".to_string()),
    };
    let _response: GetTicketResponse = get_ticket(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---