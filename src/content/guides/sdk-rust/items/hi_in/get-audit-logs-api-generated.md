## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| limit | f64 | नहीं |  |
| skip | f64 | नहीं |  |
| order | models::SortDir | नहीं |  |
| after | f64 | नहीं |  |
| before | f64 | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_audit_logs उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_audit_logs(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetAuditLogsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        limit: Some(100.0),
        skip: Some(0.0),
        order: Some(models::SortDir::Desc),
        after: Some(1622505600.0),
        before: None,
    };
    let _response: GetAuditLogsResponse = get_audit_logs(config, params).await?;
    Ok(())
}
[inline-code-end]