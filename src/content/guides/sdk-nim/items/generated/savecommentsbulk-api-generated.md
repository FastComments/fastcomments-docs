## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | seq[CreateCommentParams] | No |  |
| isLive | bool | No |  |
| doSpamCheck | bool | No |  |
| sendEmails | bool | No |  |
| populateNotifications | bool): (Option[seq[SaveComment_200_response]] | No |  |
| id | string | No |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'saveCommentsBulk Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createParams = @[
  CreateCommentParams(
    urlId = "news/2025-election-analysis",
    content = "Insightful reporting — thanks for the deep dive.",
    userId = "user-789",
    anonUserId = "",
    parentId = ""
  )
]

let unBlockParams = UnBlockFromCommentParams(commentId = "cmt-2345", reason = "User appeal accepted")

let (response, httpResponse) = client.saveCommentsBulk(
  tenantId = "my-tenant-123",
  createCommentParams = createParams,
  isLive = true,
  doSpamCheck = true,
  sendEmails = false,
  populateNotifications = true,
  id = "op-5678",
  unBlockFromCommentParams = unBlockParams,
  userId = "user-789",
  anonUserId = ""
)

if response.isSome:
  let unblockResult = response.get()
[inline-code-end]
