## Parámetros

| Name | Type | Location | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## Respuesta

Devuelve: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCount200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'getPendingWebhookEventCount Ejemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (opcional)
let externalId = "externalId_example" // String |  (opcional)
let eventType = "eventType_example" // String |  (opcional)
let type = "type_example" // String |  (opcional)
let domain = "domain_example" // String |  (opcional)
let attemptCountGT = 987 // Double |  (opcional)

DefaultAPI.getPendingWebhookEventCount(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]