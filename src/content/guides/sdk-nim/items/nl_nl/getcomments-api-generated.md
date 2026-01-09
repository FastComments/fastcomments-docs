## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | int | Nee |  |
| limit | int | Nee |  |
| skip | int | Nee |  |
| asTree | bool | Nee |  |
| skipChildren | int | Nee |  |
| limitChildren | int | Nee |  |
| maxTreeDepth | int | Nee |  |
| urlId | string | Ja |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |
| contextUserId | string | Nee |  |
| hashTag | string | Nee |  |
| parentId | string | Nee |  |
| direction | SortDirections | Nee |  |

## Respons

Retourneert: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getComments Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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