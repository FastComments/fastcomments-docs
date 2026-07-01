## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |

## Antwort

Rückgabe: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_configs_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_domain_configs Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetDomainConfigsParams {
        tenant_id: "acme-corp-tenant".to_string(),
    };
    let _response = get_domain_configs(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---