## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createCommentParams | CreateCommentParams | 否 |  |
| isLive | bool | 否 |  |
| doSpamCheck | bool | 否 |  |
| sendEmails | bool | 否 |  |
| populateNotifications | bool | 否 |  |

## 响应

返回：[`Option[SaveComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_save_comment200response.nim)

## 示例

[inline-code-attrs-start title = 'saveComment 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.saveComment(
  tenantId = "my-tenant-123",
  createCommentParams = CreateCommentParams(
    content = "This article was really helpful, thanks!",
    urlId = "news/2025-11/ai-regulations",
    authorName = "Jane Doe",
    authorEmail = "jane.doe@example.com",
    tags = @["policy", "analysis"]
  ),
  isLive = true,
  doSpamCheck = true,
  sendEmails = true,
  populateNotifications = false
)

if response.isSome:
  let saved = response.get()
  discard saved
[inline-code-end]

---