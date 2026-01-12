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
  content = "Great article — helped me understand async patterns.",
  authorName = "Alex Morgan",
  authorEmail = "alex.morgan@example.com",
  url = "news/my-company-launches-new-api",
  parentId = "",
  tags = @["api", "launch"]
)

let (response, httpResponse) = client.saveComment(
  tenantId = "my-tenant-123",
  createCommentParams = createParams,
  isLive = true,
  doSpamCheck = true,
  sendEmails = false,
  populateNotifications = true
)

if response.isSome:
  let saved = response.get()
  echo "Saved comment id: ", saved.id
[inline-code-end]
