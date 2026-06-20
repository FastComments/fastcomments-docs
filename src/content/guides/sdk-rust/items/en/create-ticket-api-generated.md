## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | Yes |  |
| create_ticket_body | models::CreateTicketBody | Yes |  |

## Response

Returns: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_ticket_response.rs)

## Example

[inline-code-attrs-start title = 'create_ticket Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
