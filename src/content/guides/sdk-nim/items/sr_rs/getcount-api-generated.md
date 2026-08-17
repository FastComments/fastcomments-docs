## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetCountOptions | No |  |

## Одговор

Враћа: [`Option[ModerationAPICountCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_count_comments_response.nim)

## Пример

[inline-code-attrs-start title = 'Primer getCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (countOpt, httpResponse) = client.getCount(tenantId = "my-tenant-123", options = GetCountOptions())
if countOpt.isSome:
  let count = countOpt.get()
[inline-code-end]