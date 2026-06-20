---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| banUserUndoParams | BanUserUndoParams | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'postBanUserUndo の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let banParams = BanUserUndoParams(
  tenantId = "my-tenant-123",
  userId = "user-987",
  undoneBy = "moderator-42",
  reason = "Reinstated after manual review"
)
let (response, httpResponse) = client.postBanUserUndo(banUserUndoParams = banParams, sso = "sso-jwt-abc123")
if response.isSome:
  let apiResp = response.get()
  echo "Ban undo succeeded, http status: " & $httpResponse.status
else:
  echo "Ban undo failed, http status: " & $httpResponse.status
[inline-code-end]

---