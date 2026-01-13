## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | No |  |

## Risposta

Restituisce: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddHashTagsBulk200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di addHashTagsBulk'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalarlo via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |  (opzionale)
let bulkCreateHashTagsBody = BulkCreateHashTagsBody(tenantId: "tenantId_example", tags: [BulkCreateHashTagsBody_tags_inner(url: "url_example", tag: "tag_example")]) // BulkCreateHashTagsBody |  (opzionale)

DefaultAPI.addHashTagsBulk(tenantId: tenantId, bulkCreateHashTagsBody: bulkCreateHashTagsBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]