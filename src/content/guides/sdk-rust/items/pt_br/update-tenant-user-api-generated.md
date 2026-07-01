## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_tenant_user_body | models::UpdateTenantUserBody | Yes |  |
| update_comments | String | No |  |

## Resposta

Retorna: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo update_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = UpdateTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        update_tenant_user_body: models::UpdateTenantUserBody {
            email: "jane.doe@example.com".to_string(),
            role: "editor".to_string(),
        },
        update_comments: Some("Promoted to editor".to_string()),
    };
    let _ = update_tenant_user(configuration, params).await?;
    Ok(())
}
[inline-code-end]