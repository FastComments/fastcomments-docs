## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| skip | float64 | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetModeratorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getModerators'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let moderators = response.get()
  echo "Received moderators response:", moderators
else:
  echo "No moderators returned"
[inline-code-end]

---