## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| userId | string | Nie |  |
| state | float64 | Nie |  |
| skip | float64 | Nie |  |
| limit | float64 | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetTickets_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTickets(tenantId = "my-tenant-123", userId = "user-456", state = 1.0, skip = 0.0, limit = 50.0)
if response.isSome:
  let tickets = response.get()
  echo tickets
else:
  echo "No tickets returned"
[inline-code-end]

---