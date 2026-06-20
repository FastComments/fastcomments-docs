## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| domain | String | Ja |  |

## Svar

Returnerer: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_config_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_domain_config Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_domain_config() -> Result<GetDomainConfigResponse, Error> {
    let tenant_id: String = "acme-corp-tenant".to_string();
    let domain_override: Option<String> = Some("news.example.com".to_string());
    let domain: String = domain_override.unwrap_or_else(|| "blog.example.com".to_string());
    let params: GetDomainConfigParams = GetDomainConfigParams { tenant_id, domain };
    let cfg: &configuration::Configuration = &configuration;
    let response: GetDomainConfigResponse = get_domain_config(cfg, params).await?;
    Ok(response)
}
[inline-code-end]

---