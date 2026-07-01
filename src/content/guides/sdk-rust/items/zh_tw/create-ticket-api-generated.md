## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| user_id | String | 是 |  |
| create_ticket_body | models::CreateTicketBody | 是 |  |

## 回應

返回：[`CreateTicketResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_ticket_response.rs)

## 範例

[inline-code-attrs-start title = 'create_ticket 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let body = models::CreateTicketBody {
        title: "Login Issue".to_string(),
        description: "User cannot log in after password reset".to_string(),
        priority: Some("high".to_string()),
    };
    let params = CreateTicketParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "user-12345".to_string(),
        create_ticket_body: body,
    };
    let _response: CreateTicketResponse = create_ticket(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---