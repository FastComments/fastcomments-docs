## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| sso | string | 否 |  |

## 回應

回傳： [`Option[APIModerateGetUserBanPreferencesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_moderate_get_user_ban_preferences_response.nim)

## 範例

[inline-code-attrs-start title = 'getUserBanPreference 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBanPreference(sso = "sso-jwt-7f3a9b")
if response.isSome:
  let prefs = response.get()
  echo "User ban preferences:", prefs
else:
  echo "No ban preference found"
[inline-code-end]

---