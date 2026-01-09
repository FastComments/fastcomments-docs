## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 回應

回傳：[`Option[UpdateUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_badge200response.nim)

## 範例

[inline-code-attrs-start title = 'deleteUserBadge 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteUserBadge(tenantId = "my-tenant-123", id = "badge-456")
if response.isSome:
  let updated = response.get()
  discard updated
[inline-code-end]

---