## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| user_id | String | 是 |  |
| id | String | 是 |  |
| change_ticket_state_body | models::ChangeTicketStateBody | 是 |  |

## 回應

回傳: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_ticket_state_200_response.rs)

## 範例

[inline-code-attrs-start title = 'change_ticket_state 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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