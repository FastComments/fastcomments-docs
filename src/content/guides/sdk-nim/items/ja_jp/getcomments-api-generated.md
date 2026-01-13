## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| page | int | いいえ |  |
| limit | int | いいえ |  |
| skip | int | いいえ |  |
| asTree | bool | いいえ |  |
| skipChildren | int | いいえ |  |
| limitChildren | int | いいえ |  |
| maxTreeDepth | int | いいえ |  |
| urlId | string | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |
| contextUserId | string | いいえ |  |
| hashTag | string | いいえ |  |
| parentId | string | いいえ |  |
| direction | SortDirections | いいえ |  |

## レスポンス

戻り値: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## 例

[inline-code-attrs-start title = 'getComments の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComments(
  tenantId = "my-tenant-123",
  page = 1,
  limit = 20,
  skip = 0,
  asTree = false,
  skipChildren = 0,
  limitChildren = 0,
  maxTreeDepth = 0,
  urlId = "news/2025-election-night",
  userId = "",
  anonUserId = "",
  contextUserId = "",
  hashTag = "",
  parentId = "",
  direction = SortDirections.Desc
)

if response.isSome:
  let comments = response.get()
  echo "Status: ", httpResponse.status, " Comments: ", comments
[inline-code-end]