---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| user_id | String | Evet |  |
| id | String | Evet |  |
| change_ticket_state_body | models::ChangeTicketStateBody | Evet |  |

## Yanıt

Döndürür: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_ticket_state_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'change_ticket_state Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<ChangeTicketState200Response, Error> {
    let params: ChangeTicketStateParams = ChangeTicketStateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "agent_jdoe".to_string(),
        id: "ticket-2026-03-25-001".to_string(),
        change_ticket_state_body: models::ChangeTicketStateBody {
            state: "closed".to_string(),
            note: Some("Issue resolved after patch deployment".to_string()),
            assignee_id: Some("agent_jdoe".to_string()),
            priority: Some("low".to_string()),
        },
    };
    let response: ChangeTicketState200Response = change_ticket_state(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---