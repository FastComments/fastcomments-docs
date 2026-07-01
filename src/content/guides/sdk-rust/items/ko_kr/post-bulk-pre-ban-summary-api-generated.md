## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| bulk_pre_ban_params | models::BulkPreBanParams | 예 |  |
| include_by_user_id_and_email | bool | 아니오 |  |
| include_by_ip | bool | 아니오 |  |
| include_by_email_domain | bool | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_pre_ban_summary.rs)

## 예시

[inline-code-attrs-start title = 'post_bulk_pre_ban_summary 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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