---
## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tag | string | Не |  |
| tenantId | string | Да |  |
| deleteHashTagRequest | DeleteHashTagRequest | Не |  |

## Одговор

Враћа: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Пример

[inline-code-attrs-start title = 'deleteHashTag пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteHashTag(tag = "breaking-news", tenantId = "my-tenant-123", deleteHashTagRequest = DeleteHashTagRequest())
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---