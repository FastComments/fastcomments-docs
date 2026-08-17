## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Не |  |
| sso | string = "" | Не |  |

## Одговор

Враћа: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Пример

[inline-code-attrs-start title = 'blockFromCommentPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (blockOpt, httpResp) = client.blockFromCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(),
  sso = ""
)

if blockOpt.isSome:
  let block = blockOpt.get()
[inline-code-end]