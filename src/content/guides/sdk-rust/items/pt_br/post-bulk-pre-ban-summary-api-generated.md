## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| bulk_pre_ban_params | models::BulkPreBanParams | Sim |  |
| include_by_user_id_and_email | bool | Não |  |
| include_by_ip | bool | Não |  |
| include_by_email_domain | bool | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_pre_ban_summary.rs)

## Exemplo

[inline-code-attrs-start title = 'post_bulk_pre_ban_summary Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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