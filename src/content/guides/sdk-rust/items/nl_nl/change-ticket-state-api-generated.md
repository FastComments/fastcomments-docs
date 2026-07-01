## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| user_id | String | Ja |  |
| id | String | Ja |  |
| change_ticket_state_body | models::ChangeTicketStateBody | Ja |  |

## Respons

Retourneert: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_ticket_state_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'change_ticket_state Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(config: &configuration::Configuration) -> Result<(), Error> {
    let body = models::ChangeTicketStateBody {
        state: Some("closed".to_string()),
        comment: Some("Issue resolved".to_string()),
    };
    let params = ChangeTicketStateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "user-1234".to_string(),
        id: "ticket-5678".to_string(),
        change_ticket_state_body: body,
    };
    let _response: ChangeTicketStateResponse = change_ticket_state(config, params).await?;
    Ok(())
}
[inline-code-end]