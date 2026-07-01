## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|------------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| sure | String | Não |  |

## Resposta

Retorna: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo delete_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn delete_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = DeleteTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        sure: Some("true".to_string()),
    };
    delete_tenant(config, params).await?;
    Ok(())
}
[inline-code-end]

---