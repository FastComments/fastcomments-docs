---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tag | string | 否 |  |
| tenantId | string | 是 |  |
| updateHashTagBody | UpdateHashTagBody | 否 |  |

## 响应

返回：[`Option[PatchHashTag_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_hash_tag200response.nim)

## 示例

[inline-code-attrs-start title = 'patchHashTag 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "politics", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())

if response.isSome:
  let updated = response.get()
  echo "Hashtag updated successfully"
else:
  echo "Failed to update hashtag, status:", httpResponse.status
[inline-code-end]

---