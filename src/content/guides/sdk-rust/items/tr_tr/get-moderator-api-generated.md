## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |

## Yanıt

Döndürür: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderator_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_moderator Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_moderator() -> Result<GetModeratorResponse, Error> {
    let params: GetModeratorParams = GetModeratorParams {
        tenant_id: "acme-newsroom".to_string(),
        id: "mod-jane-smith-001".to_string(),
    };
    let include_permissions: Option<bool> = Some(true);
    let moderator: GetModeratorResponse = get_moderator(&configuration, params).await?;
    Ok(moderator)
}
[inline-code-end]

---