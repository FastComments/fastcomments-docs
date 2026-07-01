## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | bool | No |  |
| sso | string = "" | No |  |

## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'flagCommentPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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