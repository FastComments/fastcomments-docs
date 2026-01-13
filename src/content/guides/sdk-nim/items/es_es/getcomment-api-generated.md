## Parámetros

| Name | Type | Obligatorio | Descripción |
|------|------|------------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |

## Respuesta

Devuelve: [`Option[GetComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComment(tenantId = "my-tenant-123", id = "cmt-987654321")
if response.isSome:
  let comment = response.get()
  echo comment
[inline-code-end]

---