## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| text_search | String | Não |  |
| by_ip_from_comment | String | Não |  |
| filter | String | Não |  |
| search_filters | String | Não |  |
| demo | bool | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`ModerationApiCountCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_count_comments_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("breaking news".to_string()),
        by_ip_from_comment: Some("192.168.1.1".to_string()),
        filter: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        demo: Some(false),
        sso: Some("sso-token-123".to_string()),
    };
    let _response = get_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]