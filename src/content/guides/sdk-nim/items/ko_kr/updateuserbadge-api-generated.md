## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| updateUserBadgeParams | UpdateUserBadgeParams | 아니요 |  |

## 응답

반환: [`Option[UpdateUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_badge200response.nim)

## 예제

[inline-code-attrs-start title = 'updateUserBadge 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateParams = UpdateUserBadgeParams(
  name = "Top Contributor",
  description = "Awarded for consistent high-quality comments",
  active = true,
  tags = @["community", "milestone"]
)

let (response, httpResponse) = client.updateUserBadge(tenantId = "my-tenant-123", id = "badge-456", updateUserBadgeParams = updateParams)

if response.isSome:
  let updated = response.get()
  echo "Badge updated successfully"
else:
  echo "Failed to update badge, HTTP status: ", $httpResponse.status
[inline-code-end]

---