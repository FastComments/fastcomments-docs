## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| skip | f64 | Hayır |  |

## Yanıt

Döndürür: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_moderators Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_moderators(configuration: &configuration::Configuration) -> Result<GetModeratorsResponse, Error> {
    let params: GetModeratorsParams = GetModeratorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let response: GetModeratorsResponse = get_moderators(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---