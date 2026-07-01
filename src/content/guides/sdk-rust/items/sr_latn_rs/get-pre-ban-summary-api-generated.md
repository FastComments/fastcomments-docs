## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| include_by_user_id_and_email | bool | Ne |  |
| include_by_ip | bool | Ne |  |
| include_by_email_domain | bool | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`PreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pre_ban_summary.rs)

## Primer

[inline-code-attrs-start title = 'get_pre_ban_summary Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---