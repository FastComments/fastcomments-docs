## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updatableCommentParams | UpdatableCommentParams | Ne |  |
| contextUserId | string | Ne |  |
| doSpamCheck | bool | Ne |  |
| isLive | bool | Ne |  |

## Odgovor

Vrne: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer updateComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updatableCommentParams = UpdatableCommentParams(content: "Fixed a typo in the second paragraph", tags: @["article-edit", "typo"], isApproved: true)
let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  updatableCommentParams = updatableCommentParams,
  contextUserId = "user-789",
  doSpamCheck = true,
  isLive = true
)
if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

---