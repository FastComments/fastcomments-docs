## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | float64 | Nee |  |

## Respons

Retourneert: [`Option[GetModerators_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getModerators Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let moderators = response.get()
  echo "Moderators fetched successfully"
  echo moderators
[inline-code-end]

---