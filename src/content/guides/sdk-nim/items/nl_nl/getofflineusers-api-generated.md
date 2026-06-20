Eerdere reageerders op de pagina die op dit moment NIET online zijn. Gesorteerd op displayName.
Gebruik dit nadat /users/online is uitgeput om een "Leden"-sectie weer te geven.
Cursorpaginatie op commenterName: de server doorloopt de partiële {tenantId, urlId, commenterName}
index vanaf afterName vooruit via $gt, geen $skip-kosten.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nee |  |
| afterUserId | string | Nee |  |

## Respons

Geeft terug: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-how-to-code",
  afterName = "",
  afterUserId = ""
)

if response.isSome:
  let offlinePage = response.get()
  echo "Received offline users page"
  discard httpResponse.statusCode
[inline-code-end]

---