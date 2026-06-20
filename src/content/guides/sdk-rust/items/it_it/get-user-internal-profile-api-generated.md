---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| comment_id | String | No |  |
| sso | String | No |  |

## Risposta

Restituisce: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_internal_profile_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_user_internal_profile'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_profile() -> Result<GetUserInternalProfileResponse, Error> {
    let params: GetUserInternalProfileParams = GetUserInternalProfileParams {
        comment_id: Some(String::from("cmt-72a1f4")),
        sso: Some(String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoidXNlcjEyMyJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c")),
    };
    let profile: GetUserInternalProfileResponse = get_user_internal_profile(&configuration, params).await?;
    Ok(profile)
}
[inline-code-end]

---