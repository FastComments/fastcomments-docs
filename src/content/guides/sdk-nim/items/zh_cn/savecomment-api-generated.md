---
## 参数

| Name | Type | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createCommentParams | CreateCommentParams | 否 |  |
| isLive | bool | 否 |  |
| doSpamCheck | bool | 否 |  |
| sendEmails | bool | 否 |  |
| populateNotifications | bool | 否 |  |

## 响应

返回: [`Option[APISaveCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_save_comment_response.nim)

## 示例

[inline-code-attrs-start title = 'saveComment 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createCommentParams = CreateCommentParams(
  urlId = "news/2026/major-policy-change",
  content = "This is a thoughtful comment on the policy change and its potential impacts.",
  authorName = "Morgan Lee",
  authorEmail = "morgan.lee@example.org",
  tags = @["policy","analysis"],
  extraData = @[])

let (response, httpResponse) = client.saveComment(
  tenantId = "my-tenant-123",
  createCommentParams = createCommentParams,
  isLive = true,
  doSpamCheck = true,
  sendEmails = false,
  populateNotifications = true)

if response.isSome:
  let saved = response.get()
  discard saved
[inline-code-end]

---