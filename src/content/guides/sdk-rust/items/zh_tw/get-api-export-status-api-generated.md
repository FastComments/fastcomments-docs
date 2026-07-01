## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| batch_job_id | String | No |  |
| sso | String | No |  |

## 回應

返回：[`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_status_response.rs)

## 範例

[inline-code-attrs-start title = 'get_api_export_status 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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