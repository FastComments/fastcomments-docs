Cancellazioni (unsubscribe di hook REST). Solo le sottoscrizioni create tramite questa API possono essere eliminate qui; i webhook gestiti dal dashboard vengono modificati nel dashboard.

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Example

[inline-code-attrs-start title = 'Esempio deleteWebhook'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.deleteWebhook(tenantId: tenantId, id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]