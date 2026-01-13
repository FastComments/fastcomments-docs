## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nee |  |

## Respons

Geeft terug: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddHashTagsBulk200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'addHashTagsBulk Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen, rapporteer deze via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |  (optioneel)
let bulkCreateHashTagsBody = BulkCreateHashTagsBody(tenantId: "tenantId_example", tags: [BulkCreateHashTagsBody_tags_inner(url: "url_example", tag: "tag_example")]) // BulkCreateHashTagsBody |  (optioneel)

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