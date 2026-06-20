## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tag | string | 否 |  |
| tenantId | string | 是 |  |
| updateHashTagBody | UpdateHashTagBody | 否 |  |

## 响应

返回: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## 示例

[inline-code-attrs-start title = 'patchHashTag 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())
if response.isSome:
  let updatedHashTag = response.get()
  echo updatedHashTag
[inline-code-end]

---