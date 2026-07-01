## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | GetManualBadgesForUserOptions | いいえ |  |

## レスポンス

戻り値: [`Option[GetUserManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_manual_badges_response.nim)

## 例

[inline-code-attrs-start title = 'getManualBadgesForUser の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (userBadgesOpt, httpResp) = client.getManualBadgesForUser(
  tenantId = "my-tenant-123",
  options = GetManualBadgesForUserOptions()
)
if userBadgesOpt.isSome:
  let badges = userBadgesOpt.get()
  echo badges
[inline-code-end]

---