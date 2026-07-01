## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| urlId | string | Так |  |

## Відповідь

Повертає: [`Option[GetV1PageLikes]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v1_page_likes.nim)

## Приклад

[inline-code-attrs-start title = 'getV1PageLikes Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV1PageLikes(tenantId = "my-tenant-123", urlId = "news/article-title")
if response.isSome:
  let pageLikes = response.get()
  # використовуйте pageLikes за потреби
[inline-code-end]