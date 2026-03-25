## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| skip | f64 | Não |  |

## Resposta

Retorna: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_packages_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_tenant_packages'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetTenantPackagesParams = GetTenantPackagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let packages: GetTenantPackages200Response = get_tenant_packages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---