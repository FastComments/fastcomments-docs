## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| page | int | 아니오 |  |
| limit | int | 아니오 |  |
| skip | int | 아니오 |  |
| asTree | bool | 아니오 |  |
| skipChildren | int | 아니오 |  |
| limitChildren | int | 아니오 |  |
| maxTreeDepth | int | 아니오 |  |
| urlId | string | 예 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |
| contextUserId | string | 아니오 |  |
| hashTag | string | 아니오 |  |
| parentId | string | 아니오 |  |
| direction | SortDirections | 아니오 |  |

## 응답

반환: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## 예제

[inline-code-attrs-start title = 'getComments 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---