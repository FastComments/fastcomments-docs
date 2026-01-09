## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[LockComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_lock_comment200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer lockComment poziva'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.lockComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-98765",
  broadcastId = "",
  sso = ""
)
if response.isSome:
  let lockResp = response.get()
  discard lockResp
[inline-code-end]

---