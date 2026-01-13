## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| page | float64 | No |  |

## Respuesta

Devuelve: [`Option[GetHashTags_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getHashTags'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getHashTags(tenantId = "my-tenant-123", page = 1.0)
if response.isSome:
  let tags = response.get()
  for t in tags:
    echo t
else:
  echo "no hashtags found"
[inline-code-end]

---