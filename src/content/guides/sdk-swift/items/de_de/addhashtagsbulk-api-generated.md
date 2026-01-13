## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nein |  |

## Antwort

Gibt zurück: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddHashTagsBulk200Response.swift)

## Beispiel

[inline-code-attrs-start title = 'addHashTagsBulk Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Für Probleme melden Sie diese bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |  (optional)
let bulkCreateHashTagsBody = BulkCreateHashTagsBody(tenantId: "tenantId_example", tags: [BulkCreateHashTagsBody_tags_inner(url: "url_example", tag: "tag_example")]) // BulkCreateHashTagsBody |  (optional)

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