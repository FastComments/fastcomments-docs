## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| updateUserBadgeParams | UpdateUserBadgeParams | いいえ |  |

## レスポンス

戻り値: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## 例

[inline-code-attrs-start title = 'updateUserBadge の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserBadge(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateUserBadgeParams = UpdateUserBadgeParams()
)

if response.isSome:
  let success = response.get()
  echo "Badge updated successfully"
else:
  echo "Badge update failed"
[inline-code-end]

---