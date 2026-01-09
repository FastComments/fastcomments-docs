## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| badgeId | string | 否 |  |
| displayedOnComments | bool | 否 |  |
| limit | float64 | 否 |  |
| skip | float64 | 否 |  |

## 响应

返回：[`Option[GetUserBadges_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badges200response.nim)

## 示例

[inline-code-attrs-start title = 'getUserBadges 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadges(
  tenantId = "my-tenant-123",
  userId = "user-789",
  badgeId = "top-commenter",
  displayedOnComments = true,
  limit = 50.0,
  skip = 0.0
)

if response.isSome:
  let badges = response.get()
  echo "Retrieved badges: ", $badges
[inline-code-end]

---