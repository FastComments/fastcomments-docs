---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| value | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回： [`Option[ModerationUserSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_user_search_response.nim)

## 示例

[inline-code-attrs-start title = 'getSearchUsers 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchUsers(value = "john.doe@example.com", sso = "sso-acme-789")
if response.isSome:
  let searchRes = response.get()
  echo "Search result:", searchRes
else:
  echo "No users found"
[inline-code-end]

---