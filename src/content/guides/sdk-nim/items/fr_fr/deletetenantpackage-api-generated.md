---
## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |

## Réponse

Renvoie: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantPackage(tenantId = "my-tenant-123", id = "pkg-789")
if response.isSome:
  let deleted = response.get()
  echo "Successfully deleted tenant package"
[inline-code-end]

---