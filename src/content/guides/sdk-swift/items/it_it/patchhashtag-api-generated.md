## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tag | string | path | Sì |  |
| tenantId | string | query | No |  |

## Risposta

Restituisce: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PatchHashTag200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di patchHashTag'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Gli esempi di codice seguenti sono ancora in beta. Per eventuali problemi, segnalali tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (opzionale)
let updateHashTagBody = UpdateHashTagBody(tenantId: "tenantId_example", url: "url_example", tag: "tag_example") // UpdateHashTagBody |  (opzionale)

DefaultAPI.patchHashTag(tag: tag, tenantId: tenantId, updateHashTagBody: updateHashTagBody) { (response, error) in
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