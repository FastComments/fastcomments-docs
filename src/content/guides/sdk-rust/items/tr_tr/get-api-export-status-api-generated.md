## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| batch_job_id | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_status_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_api_export_status Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetApiExportStatusParams = GetApiExportStatusParams {
        batch_job_id: Some("export-job-2026-06-19-001".to_string()),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let status: ModerationExportStatusResponse = get_api_export_status(&configuration, params).await?;
    println!("{:#?}", status);
    Ok(())
}
[inline-code-end]

---