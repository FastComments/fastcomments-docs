---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |

## Respuesta

Devuelve: [`Option[GetV2PageReacts]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_reacts.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getV2PageReacts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV2PageReacts(tenantId = "my-tenant-123", urlId = "news/elections/2026/us-primary-results")
if response.isSome:
  let reacts = response.get()
  echo "Received reacts: ", $reacts
else:
  echo "No reacts available"
[inline-code-end]

---