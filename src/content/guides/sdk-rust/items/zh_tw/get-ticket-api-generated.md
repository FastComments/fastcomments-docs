## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| user_id | String | 否 |  |

## 回應

回傳: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_ticket_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_ticket 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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