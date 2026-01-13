## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | float64 | Nee |  |

## Antwoord

Retourneert: [`Option[GetQuestionConfigs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_configs200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getQuestionConfigs Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfigs(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let configs = response.get()
  discard configs
[inline-code-end]