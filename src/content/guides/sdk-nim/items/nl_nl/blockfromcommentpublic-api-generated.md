## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Voorbeeld

[inline-code-attrs-start title = 'blockFromCommentPublic Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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