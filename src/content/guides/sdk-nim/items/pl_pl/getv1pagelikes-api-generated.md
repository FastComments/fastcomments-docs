## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |

## Odpowiedź

Zwraca: [`Option[GetV1PageLikes]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v1_page_likes.nim)

## Przykład

[inline-code-attrs-start title = 'getV1PageLikes Przykład'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV1PageLikes(tenantId = "my-tenant-123", urlId = "news/article-title")
if response.isSome:
  let pageLikes = response.get()
  # użyj pageLikes w razie potrzeby
[inline-code-end]

---