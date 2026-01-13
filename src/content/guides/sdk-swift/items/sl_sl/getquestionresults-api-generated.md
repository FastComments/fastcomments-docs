## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne |  |
| userId | string | query | Ne |  |
| startDate | string | query | Ne |  |
| questionId | string | query | Ne |  |
| questionIds | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za vsako težavo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (neobvezno)
let userId = "userId_example" // String |  (neobvezno)
let startDate = "startDate_example" // String |  (neobvezno)
let questionId = "questionId_example" // String |  (neobvezno)
let questionIds = "questionIds_example" // String |  (neobvezno)
let skip = 987 // Double |  (neobvezno)

DefaultAPI.getQuestionResults(tenantId: tenantId, urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip) { (response, error) in
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