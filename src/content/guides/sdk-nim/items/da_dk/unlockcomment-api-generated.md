## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string = "" | No |  |

## Svar

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Eksempel

[inline-code-attrs-start title = 'unLockComment Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeEmpty, httpResp) = client.unLockComment(tenantId = "my-tenant-123", commentId = "comment-456", broadcastId = "", sso = "")
if maybeEmpty.isSome:
  let emptyResp = maybeEmpty.get()
  echo "Comment unlocked"
else:
  echo "Failed to unlock comment"
[inline-code-end]