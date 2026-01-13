## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | number | query | Nee |  |

## Respons

Retourneert: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetHashTags200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getHashTags Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog steeds in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Double |  (optioneel)

DefaultAPI.getHashTags(tenantId: tenantId, page: page) { (response, error) in
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