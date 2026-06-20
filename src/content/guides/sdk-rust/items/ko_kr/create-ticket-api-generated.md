## 매개변수

| Name | Type | 필수 | 설명 |
|------|------|------|-------------|
| tenant_id | String | 예 |  |
| user_id | String | 예 |  |
| create_ticket_body | models::CreateTicketBody | 예 |  |

## 응답

반환: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_ticket_response.rs)

## 예제

[inline-code-attrs-start title = 'create_ticket 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn create_ticket_example() -> Result<CreateTicketResponse, Error> {
    let params: CreateTicketParams = CreateTicketParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "alice-7d9".to_string(),
        create_ticket_body: models::CreateTicketBody {
            subject: "Payment issue: double charge on subscription".to_string(),
            message: "I was charged twice for the July subscription. Please refund one charge.".to_string(),
            priority: Some("high".to_string()),
            tags: Some(vec!["billing".to_string(), "subscription".to_string()]),
            contact_email: Some("alice@acme-corp.com".to_string()),
        },
    };

    let response: CreateTicketResponse = create_ticket(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---