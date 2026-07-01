## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | No |  |
| blockFromCommentParams | BlockFromCommentParams | No |  |
| options | BlockUserFromCommentOptions | No |  |

## Odgovor

Vraća: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Primer

[inline-code-attrs-start title = 'blockUserFromComment Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = BlockFromCommentParams()
let opts = BlockUserFromCommentOptions()
let (blockResult, httpResp) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  blockFromCommentParams = params,
  options = opts
)
if blockResult.isSome:
  let success = blockResult.get()
  discard success
[inline-code-end]

---