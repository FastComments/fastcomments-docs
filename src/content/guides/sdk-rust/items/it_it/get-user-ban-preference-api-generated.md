---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| sso | String | No |  |

## Risposta

Restituisce: [`ApiModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_moderate_get_user_ban_preferences_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_user_ban_preference'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetUserBanPreferenceParams = GetUserBanPreferenceParams {
        sso: Some("acme-corp-tenant".to_string()),
    };
    let response: ApiModerateGetUserBanPreferencesResponse =
        get_user_ban_preference(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---