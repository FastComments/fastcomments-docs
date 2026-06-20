Massenbenutzerinformationen für einen Mandanten. Anhand der userIds werden Anzeigeinformationen aus User / SSOUser zurückgegeben.
Vom Kommentar-Widget verwendet, um Benutzer anzureichern, die gerade durch ein Presence-Ereignis erschienen sind.
Kein Seitenkontext: der Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Nein |  |

## Antwort

Gibt zurück: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getUsersInfo Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---