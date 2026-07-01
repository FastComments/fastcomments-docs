## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| add_domain_config_params | models::AddDomainConfigParams | Tak |  |

## Odpowiedź

Zwraca: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_domain_config_response.rs)

## Przykład

[inline-code-attrs-start title = 'add_domain_config Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---