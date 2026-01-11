## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | CreateCommentParams | No |  |
| isLive | bool | No |  |
| doSpamCheck | bool | No |  |
| sendEmails | bool | No |  |
| populateNotifications | bool | No |  |

## Response

Returns: [`Option[SaveComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_save_comment200response.nim)

## Example

[inline-code-attrs-start title = 'saveComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createParams = CreateCommentParams(
  content = "Great reporting — this clarified the policy changes for me.",
  authorName = "Ava Thompson",
  authorEmail = "ava.thompson@journalistmail.com",
  urlId = "news/2025/energy-incentives",
  parentId = 0,
  tags = @["energy", "policy"]
)

let (response, httpResponse) = client.saveComment(
  tenantId = "my-tenant-123",
  createCommentParams = createParams,
  isLive = true,
  doSpamCheck = true,
  sendEmails = true,
  populateNotifications = false
)

if response.isSome:
  let saved = response.get()
  echo "Comment saved for tenant my-tenant-123"
[inline-code-end]
