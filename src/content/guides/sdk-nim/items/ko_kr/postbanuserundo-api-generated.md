## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| banUserUndoParams | BanUserUndoParams | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'postBanUserUndo 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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