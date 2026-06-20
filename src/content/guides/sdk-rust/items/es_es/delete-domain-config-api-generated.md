---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| domain | String | Sí |  |

## Respuesta

Devuelve: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_domain_config_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de delete_domain_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_domain_config() -> Result<DeleteDomainConfigResponse, Error> {
    let params = DeleteDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        domain: "news/acme-corp".to_owned(),
        force: Some(true),
    };
    let response: DeleteDomainConfigResponse = delete_domain_config(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---