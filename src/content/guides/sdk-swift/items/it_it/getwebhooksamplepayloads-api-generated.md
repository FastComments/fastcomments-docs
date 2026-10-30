Recent commenti nello stesso formato usato dalle consegne webhook, per costruire integrazioni (ad esempio dati di esempio di Zapier). Ogni evento fornisce lo stesso oggetto commento, quindi `event` deve semplicemente essere valido.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|---------------|-------------|
| tenantId | string | query | Sì |  |
| event | string | query | No |  |
| limit | number | query | No |  |

## Risposta

Restituisce: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhookSamplePayloadsResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getWebhookSamplePayloads'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Il seguente esempio di codice è ancora in beta. Per qualsiasi problema, segnalalo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
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