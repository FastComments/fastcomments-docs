## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| sso | string | 아니오 |  |

## 응답

반환: [`Option[APIModerateGetUserBanPreferencesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_moderate_get_user_ban_preferences_response.nim)

## 예제

[inline-code-attrs-start title = 'getUserBanPreference 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBanPreference(sso = "sso-jwt-7f3a9b")
if response.isSome:
  let prefs = response.get()
  echo "User ban preferences:", prefs
else:
  echo "No ban preference found"
[inline-code-end]

---