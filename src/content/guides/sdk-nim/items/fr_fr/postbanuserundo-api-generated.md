## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| banUserUndoParams | BanUserUndoParams | Non |  |
| sso | string = "" | Non |  |

## Réponse

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple postBanUserUndo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.postBanUserUndo(
  tenantId = "my-tenant-123",
  banUserUndoParams = BanUserUndoParams(userId = "user-456"),
  sso = ""
)

if apiResp.isSome:
  let _ = apiResp.get()
[inline-code-end]