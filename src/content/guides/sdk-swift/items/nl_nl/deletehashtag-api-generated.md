## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tag | string | path | Ja |  |
| tenantId | string | query | Nee |  |

## Antwoord

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'deleteHashTag Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen, meld deze via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (optioneel)
let deleteHashTagRequestBody = DeleteHashTagRequestBody(tenantId: "tenantId_example") // DeleteHashTagRequestBody |  (optioneel)

DefaultAPI.deleteHashTag(tag: tag, tenantId: tenantId, deleteHashTagRequestBody: deleteHashTagRequestBody) { (response, error) in
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