## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |

## Réponse

Renvoie: [`Option[GetUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUser(tenantId = "my-tenant-123", id = "user-456")
if response.isSome:
  let user = response.get()
  echo user
else:
  echo "User not found"
[inline-code-end]

---