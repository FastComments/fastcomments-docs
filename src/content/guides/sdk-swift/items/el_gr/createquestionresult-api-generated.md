## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateQuestionResult200Response.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createQuestionResult'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα παραδείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createQuestionResultBody = CreateQuestionResultBody(urlId: "urlId_example", value: 123, questionId: "questionId_example", anonUserId: "anonUserId_example", userId: "userId_example", commentId: "commentId_example", meta: [MetaItem(name: "name_example", values: ["values_example"])]) // CreateQuestionResultBody | 

DefaultAPI.createQuestionResult(tenantId: tenantId, createQuestionResultBody: createQuestionResultBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]