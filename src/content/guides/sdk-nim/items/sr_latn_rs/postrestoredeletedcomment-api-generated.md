## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostRestoreDeletedCommentOptions | No |  |

## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'postRestoreDeletedComment Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.postRestoreDeletedComment(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  options = default(PostRestoreDeletedCommentOptions)
)

if respOpt.isSome:
  let empty = respOpt.get()
  echo "Comment restored"
[inline-code-end]