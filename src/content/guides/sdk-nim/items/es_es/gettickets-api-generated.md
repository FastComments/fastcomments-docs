## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|--------------|
| tenantId | string | Yes |  |
| options | GetTicketsOptions | No |  |

## Respuesta

Devuelve: [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketsOpt, httpResp) = client.getTickets(tenantId = "my-tenant-123", options = GetTicketsOptions())
if ticketsOpt.isSome:
  let tickets = ticketsOpt.get()
  # usar tickets según sea necesario
[inline-code-end]

---