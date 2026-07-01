## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| comment_id | String | Nee |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_internal_profile_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_user_internal_profile voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_profile() -> Result<(), Error> {
    let params = GetUserInternalProfileParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: Some("news/article".to_string()),
        sso: Some("sso-user-xyz".to_string()),
    };
    let _response = get_user_internal_profile(&configuration, params).await?;
    Ok(())
}
[inline-code-end]