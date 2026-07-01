## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| bulk_pre_ban_params | models::BulkPreBanParams | 是 |  |
| include_by_user_id_and_email | bool | 否 |  |
| include_by_ip | bool | 否 |  |
| include_by_email_domain | bool | 否 |  |
| sso | String | 否 |  |

## 回應

返回：[`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_pre_ban_summary.rs)

## 範例

[inline-code-attrs-start title = 'post_bulk_pre_ban_summary 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let bulk_params = models::BulkPreBanParams::default();
    let params = PostBulkPreBanSummaryParams {
        tenant_id: "acme-corp-tenant".to_string(),
        bulk_pre_ban_params: bulk_params,
        include_by_user_id_and_email: Some(true),
        include_by_ip: Some(false),
        include_by_email_domain: Some(true),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _summary = post_bulk_pre_ban_summary(&configuration, params).await?;
    Ok(())
}
[inline-code-end]