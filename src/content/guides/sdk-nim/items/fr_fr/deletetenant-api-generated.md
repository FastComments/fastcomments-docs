## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |
| sure | string | Non |  |

## Réponse

Retourne : [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'utilisation de deleteTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenant(tenantId = "my-tenant-123", id = "", sure = "")

if response.isSome:
  let flagResp = response.get()
  echo "Tenant deletion response received for tenant: ", "my-tenant-123"
  discard flagResp
else:
  echo "No response body returned for tenant deletion"
[inline-code-end]

---