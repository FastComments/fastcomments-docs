## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| comment_id | String | Sim |  |
| include_by_user_id_and_email | bool | Não |  |
| include_by_ip | bool | Não |  |
| include_by_email_domain | bool | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`PreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pre_ban_summary.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_pre_ban_summary'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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