## Параметри

| Name | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| page | int | Ні |  |
| limit | int | Ні |  |
| skip | int | Ні |  |
| asTree | bool | Ні |  |
| skipChildren | int | Ні |  |
| limitChildren | int | Ні |  |
| maxTreeDepth | int | Ні |  |
| urlId | string | Так |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |
| contextUserId | string | Ні |  |
| hashTag | string | Ні |  |
| parentId | string | Ні |  |
| direction | SortDirections | Ні |  |

## Відповідь

Повертає: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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