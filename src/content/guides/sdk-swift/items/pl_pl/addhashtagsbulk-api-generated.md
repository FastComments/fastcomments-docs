## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddHashTagsBulk200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład addHashTagsBulk'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w fazie beta. W razie problemu zgłoś go pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |  (opcjonalne)
let bulkCreateHashTagsBody = BulkCreateHashTagsBody(tenantId: "tenantId_example", tags: [BulkCreateHashTagsBody_tags_inner(url: "url_example", tag: "tag_example")]) // BulkCreateHashTagsBody |  (opcjonalne)

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

---