Tidligere kommentatorer på siden, som IKKE er online i øjeblikket. Sorteret efter displayName.
Brug denne efter at have brugt /users/online op til at rendere et "Members"-afsnit.
Cursor-paginering på commenterName: serveren gennemløber det partielle {tenantId, urlId, commenterName}
indeks fra afterName fremad via $gt, uden $skip-omkostninger.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| options | GetOfflineUsersOptions | Nej |  |

## Svar

Returnerer: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]