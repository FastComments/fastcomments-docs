## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | float64 | Nej |  |

## Svar

Returnerer: [`Option[GetQuestionConfigs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_configs200response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getQuestionConfigs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfigs(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let configs = response.get()
  discard configs
[inline-code-end]

---