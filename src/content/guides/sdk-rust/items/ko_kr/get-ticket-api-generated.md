## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| user_id | String | 아니요 |  |

## 응답

반환: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_ticket_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_ticket 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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