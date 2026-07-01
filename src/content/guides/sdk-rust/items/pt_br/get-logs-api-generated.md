## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| comment_id | String | Sim |  |
| sso | String | Não |  |

## Resposta

Retorna: [`ModerationApiGetLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_logs_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_logs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetLogsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-12345".to_string(),
        sso: Some("user@example.com".to_string()),
    };
    let response = get_logs(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---