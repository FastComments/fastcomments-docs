## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | int | Нет |  |
| limit | int | Нет |  |
| skip | int | Нет |  |
| asTree | bool | Нет |  |
| skipChildren | int | Нет |  |
| limitChildren | int | Нет |  |
| maxTreeDepth | int | Нет |  |
| urlId | string | Да |  |
| userId | string | Нет |  |
| anonUserId | string | Нет |  |
| contextUserId | string | Нет |  |
| hashTag | string | Нет |  |
| parentId | string | Нет |  |
| direction | SortDirections | Нет |  |

## Ответ

Возвращает: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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