## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Respons

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'updateQuestionResult Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateQuestionResultBody = UpdateQuestionResultBody(urlId: "urlId_example", anonUserId: "anonUserId_example", userId: "userId_example", value: 123, commentId: "commentId_example", questionId: "questionId_example", meta: [MetaItem(name: "name_example", values: ["values_example"])]) // UpdateQuestionResultBody | 

DefaultAPI.updateQuestionResult(tenantId: tenantId, id: id, updateQuestionResultBody: updateQuestionResultBody) { (response, error) in
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