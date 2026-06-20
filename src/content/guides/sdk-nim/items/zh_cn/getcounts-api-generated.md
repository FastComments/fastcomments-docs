## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| sso | string | 否 |  |

## 响应

返回: [`Option[GetBannedUsersCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_count_response.nim)

## 示例

[inline-code-attrs-start title = 'getCounts 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCounts(sso = "sso_my-tenant-123_token_AbCdEf123456")
if response.isSome:
  let counts = response.get()
  echo counts
else:
  echo "Request failed with status:", httpResponse.status
[inline-code-end]

---