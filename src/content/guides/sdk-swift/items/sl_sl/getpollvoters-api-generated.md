Who voted for which option. Anonymous polls never answer, admins-only polls answer only for an admin or moderator.

## Parameters

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| urlId | string | query | No |  |
| page | integer | query | No |  |
| search | string | query | No |  |
| sso | string | query | No |  |

## Response

Vrne: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## Example

[inline-code-attrs-start title = 'getPollVoters Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za kakršnekoli težave, prosimo, poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String |  (neobvezno)
let page = 987 // Int |  (neobvezno)
let search = "search_example" // String |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)

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