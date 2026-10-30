Recent comments exactly in the format webhook deliveries use, for building integrations (for example Zapier sample data). Every event delivers the same comment object, so `event` only has to be valid.

## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|------------|-----------|
| tenantId | string | query | Sim |  |
| event | string | query | Não |  |
| limit | number | query | Não |  |

## Resposta

Retorna: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhookSamplePayloadsResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getWebhookSamplePayloads'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (optional)
let limit = 987 // Double |  (optional)

DefaultAPI.getWebhookSamplePayloads(tenantId: tenantId, options: DefaultAPI.GetWebhookSamplePayloadsOptions(event: event, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---