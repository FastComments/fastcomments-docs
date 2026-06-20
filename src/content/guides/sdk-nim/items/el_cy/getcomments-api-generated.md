## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| page | int | Όχι |  |
| limit | int | Όχι |  |
| skip | int | Όχι |  |
| asTree | bool | Όχι |  |
| skipChildren | int | Όχι |  |
| limitChildren | int | Όχι |  |
| maxTreeDepth | int | Όχι |  |
| urlId | string | Ναι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |
| contextUserId | string | Όχι |  |
| hashTag | string | Όχι |  |
| parentId | string | Όχι |  |
| direction | SortDirections | Όχι |  |
| fromDate | int64 | Όχι |  |
| toDate | int64 | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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