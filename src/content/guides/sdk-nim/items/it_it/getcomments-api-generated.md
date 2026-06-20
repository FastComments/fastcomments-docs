---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| page | int | No |  |
| limit | int | No |  |
| skip | int | No |  |
| asTree | bool | No |  |
| skipChildren | int | No |  |
| limitChildren | int | No |  |
| maxTreeDepth | int | No |  |
| urlId | string | Sì |  |
| userId | string | No |  |
| anonUserId | string | No |  |
| contextUserId | string | No |  |
| hashTag | string | No |  |
| parentId | string | No |  |
| direction | SortDirections | No |  |
| fromDate | int64 | No |  |
| toDate | int64 | No |  |

## Risposta

Restituisce: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComments(
  tenantId = "my-tenant-123",
  page = 1,
  limit = 25,
  skip = 0,
  asTree = true,
  skipChildren = 0,
  limitChildren = 5,
  maxTreeDepth = 3,
  urlId = "news/2026-global-economy",
  userId = "user-789",
  anonUserId = "",
  contextUserId = "",
  hashTag = "economy",
  parentId = "",
  direction = SortDirections.Desc,
  fromDate = 1710000000000'i64,
  toDate = 1710100000000'i64
)
if response.isSome:
  let commentsResp = response.get()
  discard commentsResp
[inline-code-end]

---