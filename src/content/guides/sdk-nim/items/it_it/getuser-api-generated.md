---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |

## Risposta

Restituisce: [`Option[GetUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUser(tenantId = "my-tenant-123", id = "user-9876")
if response.isSome:
  let user = response.get()
  echo "User:", user
else:
  echo "No user found. HTTP response:", httpResponse
[inline-code-end]

---