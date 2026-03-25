## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |

## Resposta

Retorna: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_badge_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de delete_user_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_badge() -> Result<UpdateUserBadge200Response, Error> {
    let params: DeleteUserBadgeParams = DeleteUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-7890".to_string(),
    };
    let response: UpdateUserBadge200Response = delete_user_badge(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---