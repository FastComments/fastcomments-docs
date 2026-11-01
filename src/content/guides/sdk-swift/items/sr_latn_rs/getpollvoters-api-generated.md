Ko je glasao za koju opciju. Anonimne ankete nikada ne odgovaraju, ankete samo za administratore odgovaraju samo administratoru ili moderatoru.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| urlId | string | query | No |  |
| page | integer | query | No |  |
| search | string | query | No |  |
| sso | string | query | No |  |

## Response

Vraća: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## Example

[inline-code-attrs-start title = 'Primer getPollVoters'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String |  (optional)
let page = 987 // Int |  (optional)
let search = "search_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.getPollVoters(tenantId: tenantId, commentId: commentId, options: PublicAPI.GetPollVotersOptions(urlId: urlId, page: page, search: search, sso: sso)) { (response, error) in
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