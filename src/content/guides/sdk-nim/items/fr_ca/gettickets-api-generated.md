## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| state | float64 | Non |  |
| skip | float64 | Non |  |
| limit | float64 | Non |  |

## Réponse

Renvoie : [`Option[GetTickets_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d'utilisation de getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTickets(tenantId = "my-tenant-123", userId = "user-456", state = 1.0, skip = 0.0, limit = 50.0)
if response.isSome:
  let tickets = response.get()
  echo tickets
else:
  echo "No tickets returned"
[inline-code-end]

---