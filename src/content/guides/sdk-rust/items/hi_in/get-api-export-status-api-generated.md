## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| batch_job_id | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_status_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_api_export_status उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetApiExportStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        batch_job_id: Some("batch-2023-09-01".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _status = get_api_export_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]