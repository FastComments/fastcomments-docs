---
Bulk gebruikersinformatie voor een tenant. Gegeven userIds, retourneer weergave-informatie van User / SSOUser.
Wordt gebruikt door de commentaar-widget om gebruikers te verrijken die zojuist via een presence event verschenen.
Geen paginacontext: privacy wordt uniform afgedwongen (privéprofielen worden gemaskeerd).

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Nee |  |

## Antwoord

Retourneert: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getUsersInfo Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---