## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| banUserUndoParams | BanUserUndoParams | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de postBanUserUndo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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