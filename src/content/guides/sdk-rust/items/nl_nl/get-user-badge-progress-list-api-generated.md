## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenant_id | String | Ja |  |
| user_id | String | Nee |  |
| limit | f64 | Nee |  |
| skip | f64 | Nee |  |

## Respons

Retourneert: [`ApiGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_list_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_user_badge_progress_list Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badge_progress(conf: &configuration::Configuration) -> Result<(), Error> {
    let params = GetUserBadgeProgressListParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-98765".to_string()),
        limit: Some(20.0),
        skip: Some(5.0),
    };
    let _resp = get_user_badge_progress_list(conf, params).await?;
    Ok(())
}
[inline-code-end]