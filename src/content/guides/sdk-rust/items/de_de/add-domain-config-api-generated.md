## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Yes |  |
| add_domain_config_params | models::AddDomainConfigParams | Yes |  |

## Antwort

Rückgabe: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_domain_config_response.rs)

## Beispiel

[inline-code-attrs-start title = 'add_domain_config Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = AddDomainConfigParams {
    tenant_id: "acme-corp-tenant".to_string(),
    add_domain_config_params: models::AddDomainConfigParams {
        domain: "news.example.com".to_string(),
        config_type: "article".to_string(),
        is_active: true,
        description: Some("News article domain".to_string()),
    },
};

let response = add_domain_config(&configuration, params).await?;
[inline-code-end]