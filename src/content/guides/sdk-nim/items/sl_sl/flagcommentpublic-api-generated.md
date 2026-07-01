## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | bool | No |  |
| sso | string = "" | No |  |

## Odgovor

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer flagCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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