## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | bool | No |  |
| sso | string = "" | No |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'flagCommentPublic Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.flagCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  isFlagged = true,
  sso = ""
)

if optResp.isSome:
  let empty = optResp.get()
  discard empty
[inline-code-end]