## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| comment_id | String | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_internal_profile_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_internal_profile'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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