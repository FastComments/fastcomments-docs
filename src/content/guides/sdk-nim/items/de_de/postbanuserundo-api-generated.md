## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| banUserUndoParams | BanUserUndoParams | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'postBanUserUndo Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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