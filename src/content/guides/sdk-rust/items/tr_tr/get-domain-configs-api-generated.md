## Parametreler

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |

## Yanıt

Döndürür: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_configs_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_domain_configs Örnek'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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