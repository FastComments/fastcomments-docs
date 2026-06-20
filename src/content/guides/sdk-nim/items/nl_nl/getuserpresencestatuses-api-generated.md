## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlIdWS | string | Nee |  |
| userIds | string | Nee |  |

## Antwoord

Retourneert: [`Option[GetUserPresenceStatusesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_presence_statuses_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getUserPresenceStatuses Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserPresenceStatuses(tenantId = "my-tenant-123", urlIdWS = "news/article-title", userIds = "user-123,user-456")
if response.isSome:
  let presenceStatuses = response.get()
  echo presenceStatuses
else:
  echo "No presence data"
[inline-code-end]

---