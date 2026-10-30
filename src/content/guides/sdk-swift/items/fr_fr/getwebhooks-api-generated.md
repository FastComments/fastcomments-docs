---
Liste les webhooks configurés pour le locataire, à la fois les lignes gérées via le tableau de bord et les abonnements API.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| event | string | query | Non |  |
| domain | string | query | Non |  |
| source | string | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Renvoie : [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (optional)
let domain = "domain_example" // String |  (optional)
let source = WebhookSource() // WebhookSource |  (optional)
let skip = 987 // Double |  (optional)

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

---