## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| user_id | String | Não |  |
| limit | f64 | Não |  |
| skip | f64 | Não |  |

## Resposta

Retorna: [`ApiGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_list_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_user_badge_progress_list'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetUserBadgeProgressListParams = GetUserBadgeProgressListParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-9876".to_string()),
        limit: Some(25.0),
        skip: Some(0.0),
    };
    let badge_progress: ApiGetUserBadgeProgressListResponse =
        get_user_badge_progress_list(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---