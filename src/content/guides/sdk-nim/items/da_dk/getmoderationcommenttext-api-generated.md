## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| sso | string = "" | Nej |  |

## Response

Returnerer: [`Option[GetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getModerationCommentText Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getModerationCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-456abc",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]

---