## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | int | Не |  |
| limit | int | Не |  |
| skip | int | Не |  |
| asTree | bool | Не |  |
| skipChildren | int | Не |  |
| limitChildren | int | Не |  |
| maxTreeDepth | int | Не |  |
| urlId | string | Да |  |
| userId | string | Не |  |
| anonUserId | string | Не |  |
| contextUserId | string | Не |  |
| hashTag | string | Не |  |
| parentId | string | Не |  |
| direction | SortDirections | Не |  |

## Одговор

Враћа: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## Примјер

[inline-code-attrs-start title = 'getComments Примјер'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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