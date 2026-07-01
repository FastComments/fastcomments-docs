## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| sso | string = "" | Non |  |

## Réponse

Renvoie : [`Option[APIModerateGetUserBanPreferencesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_moderate_get_user_ban_preferences_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserBanPreference'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybePref, httpResp) = client.getUserBanPreference(tenantId = "my-tenant-123", sso = "")
if maybePref.isSome:
  let pref = maybePref.get()
  echo pref
[inline-code-end]