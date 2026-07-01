## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | Yes |  |
| id | String | Yes |  |
| change_ticket_state_body | models::ChangeTicketStateBody | Yes |  |

## 回應

返回：[`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_ticket_state_response.rs)

## 範例

[inline-code-attrs-start title = 'change_ticket_state 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---