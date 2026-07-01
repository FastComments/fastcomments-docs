## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string = "" | No |  |

## Ответ

Возвращает: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getCommentChildren'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (childRespOpt, httpResp) = client.getCommentChildren(tenantId = "my-tenant-123", commentId = "cmt-456789", sso = "")
if childRespOpt.isSome:
  let childResp = childRespOpt.get()
  echo childResp
[inline-code-end]