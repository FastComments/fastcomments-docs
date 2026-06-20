## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di blockFromCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let publicParams = PublicBlockFromCommentParams(
  reason = "Repeated spam links",
  durationMinutes = 1440,
  blockAll = true,
  notifyUser = false,
  tags = @["spam", "auto-block"]
)

let (response, httpResponse) = client.blockFromCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-98765",
  publicBlockFromCommentParams = publicParams,
  sso = ""
)

if response.isSome:
  let blockResult = response.get()
  echo "Block succeeded: ", $blockResult
else:
  echo "Block failed, HTTP status: ", $httpResponse.status
[inline-code-end]