## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| sso | string = "" | 否 |  |

## 响应

返回: [`Option[GetBannedUsersCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_count_response.nim)

## 示例

[inline-code-attrs-start title = 'getCounts 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeCounts, httpRes) = client.getCounts(tenantId = "my-tenant-123", sso = "")
if maybeCounts.isSome:
  let counts = maybeCounts.get()
  echo counts
else:
  echo "No counts returned"
[inline-code-end]

---