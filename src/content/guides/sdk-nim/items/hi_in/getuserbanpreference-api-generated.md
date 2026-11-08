## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[APIModerateGetUserBanPreferencesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_moderate_get_user_ban_preferences_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getUserBanPreference उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybePref, httpResp) = client.getUserBanPreference(tenantId = "my-tenant-123", sso = "")
if maybePref.isSome:
  let pref = maybePref.get()
  echo pref
[inline-code-end]