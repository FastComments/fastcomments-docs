## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| sso | string = "" | Nee |  |

## Respons

Retourneert: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getCommentChildren'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (childRespOpt, httpResp) = client.getCommentChildren(tenantId = "my-tenant-123", commentId = "cmt-456789", sso = "")
if childRespOpt.isSome:
  let childResp = childRespOpt.get()
  echo childResp
[inline-code-end]