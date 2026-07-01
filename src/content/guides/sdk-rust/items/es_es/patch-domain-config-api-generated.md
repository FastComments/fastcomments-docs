## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenant_id | String | Sí |  |
| domain_to_update | String | Sí |  |
| patch_domain_config_params | models::PatchDomainConfigParams | Sí |  |

## Respuesta

Devuelve: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_domain_config_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de patch_domain_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = PatchDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news/article".to_string(),
        patch_domain_config_params: models::PatchDomainConfigParams {
            enable_comments: Some(true),
            theme: Some("dark".to_string()),
        },
    };
    let _response = patch_domain_config(&config, params).await?;
    Ok(())
}
[inline-code-end]