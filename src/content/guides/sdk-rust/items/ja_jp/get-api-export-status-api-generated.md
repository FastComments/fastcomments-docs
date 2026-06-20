## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| batch_job_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

返却値: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_status_response.rs)

## 例

[inline-code-attrs-start title = 'get_api_export_status の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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