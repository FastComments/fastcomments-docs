## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| userId | string | No |  |
| state | float64 | No |  |
| skip | float64 | No |  |
| limit | float64 | No |  |

## Respuesta

Devuelve: [`Option[GetTickets_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTickets(tenantId = "my-tenant-123", userId = "user-456", state = 1.0, skip = 0.0, limit = 50.0)
if response.isSome:
  let tickets = response.get()
  echo tickets
else:
  echo "No tickets returned"
[inline-code-end]

---