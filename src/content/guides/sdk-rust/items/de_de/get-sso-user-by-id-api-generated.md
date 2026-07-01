## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Antwort

Rückgabe: [`GetSsoUserByIdApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_user_by_id_api_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_sso_user_by_id Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSsoUserByIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
    };
    let _response: GetSsoUserByIdApiResponse = get_sso_user_by_id(configuration, params).await?;
    Ok(())
}
[inline-code-end]