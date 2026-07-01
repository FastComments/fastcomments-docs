## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| batch_job_id | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Rückgabe: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_status_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_api_export_status Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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