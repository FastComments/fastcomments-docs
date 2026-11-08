## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| user_id | String | नहीं |  |
| state | f64 | नहीं |  |
| skip | f64 | नहीं |  |
| limit | f64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_tickets उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_tickets(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetTicketsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-12345".to_string()),
        state: Some(1.0),
        skip: Some(0.0),
        limit: Some(20.0),
    };
    let _response: GetTicketsResponse = get_tickets(configuration, params).await?;
    Ok(())
}
[inline-code-end]