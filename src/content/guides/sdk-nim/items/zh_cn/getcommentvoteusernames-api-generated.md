---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| dir | int | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`Option[GetCommentVoteUserNames_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_vote_user_names200response.nim)

## 示例

[inline-code-attrs-start title = 'getCommentVoteUserNames 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentVoteUserNames(tenantId = "my-tenant-123", commentId = "c_987654321", dir = 0, sso = "")
if response.isSome:
  let res = response.get()
  for userName in res.userNames:
    echo userName
[inline-code-end]

---