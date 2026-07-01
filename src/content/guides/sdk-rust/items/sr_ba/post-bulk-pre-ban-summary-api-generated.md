## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| bulk_pre_ban_params | models::BulkPreBanParams | Yes |  |
| include_by_user_id_and_email | bool | No |  |
| include_by_ip | bool | No |  |
| include_by_email_domain | bool | No |  |
| sso | String | No |  |

## Odgovor

Vraća: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_pre_ban_summary.rs)

## Primjer

[inline-code-attrs-start title = 'post_bulk_pre_ban_summary Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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