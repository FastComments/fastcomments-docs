## Parámetros

| Name | Type | Requerido | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respuesta

Devuelve: [`Option[FlagComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unFlagComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unFlagComment(
  tenantId = "my-tenant-123",
  id = "flag-789",
  userId = "",
  anonUserId = ""
)

if response.isSome:
  let flagResponse = response.get()
  echo "Comment unflagged successfully"
[inline-code-end]

---