Lists the webhooks configurados para o tenant, tanto linhas gerenciadas pelo painel quanto assinaturas de API.

## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-----------|
| tenantId | string | query | Sim |  |
| event | string | query | Não |  |
| domain | string | query | Não |  |
| source | string | query | Não |  |
| skip | number | query | Não |  |

## Response

Retorna: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemplo getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// O seguinte exemplo de código ainda está em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (opcional)
let domain = "domain_example" // String |  (opcional)
let source = WebhookSource() // WebhookSource |  (opcional)
let skip = 987 // Double |  (opcional)

DefaultAPI.getWebhooks(tenantId: tenantId, options: DefaultAPI.GetWebhooksOptions(event: event, domain: domain, source: source, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]