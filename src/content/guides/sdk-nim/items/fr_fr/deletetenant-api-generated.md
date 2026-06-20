## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |
| sure | string | Non |  |

## Réponse

Retourne : [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenant(tenantId = "my-tenant-123", id = "", sure = "")
if response.isSome:
  let emptyResp = response.get()
else:
  discard httpResponse
[inline-code-end]

---