## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 否 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回：[`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## 示例

[inline-code-attrs-start title = 'unBlockUserFromComment 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-9f3b2a",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-1024",
  anonUserId = "anon-77b"
)

if response.isSome:
  let unblockResult = response.get()
  echo unblockResult
else:
  echo "Unblock failed"
[inline-code-end]

---