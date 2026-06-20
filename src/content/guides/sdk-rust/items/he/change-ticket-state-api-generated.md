## פרמטרים

| שם | Type | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| user_id | String | כן |  |
| id | String | כן |  |
| change_ticket_state_body | models::ChangeTicketStateBody | כן |  |

## תגובה

מחזיר: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_ticket_state_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-change_ticket_state'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let change_ticket_state_body: models::ChangeTicketStateBody = models::ChangeTicketStateBody {
    state: Some("resolved".to_string()),
    comment: Some("Fixed in release 1.2.3".to_string()),
    notify_subscribers: Some(true),
};

let params: ChangeTicketStateParams = ChangeTicketStateParams {
    tenant_id: "acme-corp-tenant".to_string(),
    user_id: "john.doe@acme.com".to_string(),
    id: "ticket-98765".to_string(),
    change_ticket_state_body,
};

let response: ChangeTicketStateResponse = change_ticket_state(configuration, params).await?;
[inline-code-end]

---