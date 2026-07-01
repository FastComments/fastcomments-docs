## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| sendEmail | string = "" | No |  |

## Réponse

Renvoie : [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteModerator(
  tenantId = "my-tenant-123",
  id = "mod-789",
  sendEmail = "admin@mydomain.com",
)

if apiResp.isSome:
  let empty = apiResp.get()
  echo "Moderator removed"
[inline-code-end]

---