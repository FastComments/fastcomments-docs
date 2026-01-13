## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tag | string | Ні |  |
| tenantId | string | Так |  |
| deleteHashTagRequest | DeleteHashTagRequest | Ні |  |

## Відповідь

Повертає: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteHashTag(tag = "breaking-news", tenantId = "my-tenant-123", deleteHashTagRequest = DeleteHashTagRequest())
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---