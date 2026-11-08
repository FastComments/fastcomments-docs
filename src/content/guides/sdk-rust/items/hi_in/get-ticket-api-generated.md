## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| user_id | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_ticket_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_ticket उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetTicketParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "ticket-456".to_string(),
        user_id: Some("user-123".to_string()),
    };
    let _response: GetTicketResponse = get_ticket(&configuration, params).await?;
    Ok(())
}
[inline-code-end]