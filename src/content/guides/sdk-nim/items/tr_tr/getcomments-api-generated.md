---
## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| page | int | Hayır |  |
| limit | int | Hayır |  |
| skip | int | Hayır |  |
| asTree | bool | Hayır |  |
| skipChildren | int | Hayır |  |
| limitChildren | int | Hayır |  |
| maxTreeDepth | int | Hayır |  |
| urlId | string | Evet |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |
| contextUserId | string | Hayır |  |
| hashTag | string | Hayır |  |
| parentId | string | Hayır |  |
| direction | SortDirections | Hayır |  |
| fromDate | int64 | Hayır |  |
| toDate | int64 | Hayır |  |

## Yanıt

Döndürür: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Örnek

[inline-code-attrs-start title = 'getComments Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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