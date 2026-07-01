## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| comment_id | String | Evet |  |
| include_by_user_id_and_email | bool | Hayır |  |
| include_by_ip | bool | Hayır |  |
| include_by_email_domain | bool | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`PreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pre_ban_summary.rs)

## Örnek

[inline-code-attrs-start title = 'get_pre_ban_summary Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetPreBanSummaryParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        include_by_user_id_and_email: Some(true),
        include_by_ip: Some(false),
        include_by_email_domain: Some(true),
        sso: Some("sso-token-abc".to_string()),
    };
    let _summary = get_pre_ban_summary(&configuration, params).await?;
    Ok(())
}
[inline-code-end]