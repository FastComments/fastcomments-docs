## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string = "" | No |  |

## Odgovor

Vraća: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getCommentChildren'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (childRespOpt, httpResp) = client.getCommentChildren(tenantId = "my-tenant-123", commentId = "cmt-456789", sso = "")
if childRespOpt.isSome:
  let childResp = childRespOpt.get()
  echo childResp
[inline-code-end]