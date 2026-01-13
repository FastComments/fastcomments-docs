## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nein |  |

## Antwort

Gibt zurück: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddHashTag200Response.swift)

## Beispiel

[inline-code-attrs-start title = 'addHashTag Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie sich bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |  (optional)
let createHashTagBody = CreateHashTagBody(tenantId: "tenantId_example", tag: "tag_example", url: "url_example") // CreateHashTagBody |  (optional)

DefaultAPI.addHashTag(tenantId: tenantId, createHashTagBody: createHashTagBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]