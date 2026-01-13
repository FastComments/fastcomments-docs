## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tag | string | 否 |  |
| tenantId | string | 是 |  |
| deleteHashTagRequest | DeleteHashTagRequest | 否 |  |

## 响应

返回：[`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 示例

[inline-code-attrs-start title = 'deleteHashTag 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteHashTag(tag = "breaking-news", tenantId = "my-tenant-123", deleteHashTagRequest = DeleteHashTagRequest())
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---