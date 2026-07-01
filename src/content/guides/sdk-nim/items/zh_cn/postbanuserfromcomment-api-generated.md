## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostBanUserFromCommentOptions | No |  |

## 响应

返回: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## 示例

[inline-code-attrs-start title = 'postBanUserFromComment 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (banResult, httpResp) = client.postBanUserFromComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456",
  options = PostBanUserFromCommentOptions()
)

if banResult.isSome:
  let result = banResult.get()
  echo result
[inline-code-end]

---