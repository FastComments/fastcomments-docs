## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Response

Retourneert: [`ApiGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_badge Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badge() -> Result<(), Error> {
    let params = GetUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-42".to_string(),
    };
    let _response: ApiGetUserBadgeResponse = get_user_badge(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---