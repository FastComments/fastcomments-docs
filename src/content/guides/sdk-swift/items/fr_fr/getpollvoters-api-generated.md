Who voted for which option. Anonymous polls never answer, admins-only polls answer only for an admin or moderator.

## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | chaîne | chemin | Oui |  |
| commentId | chaîne | chemin | Oui |  |
| urlId | chaîne | requête | Non |  |
| page | entier | requête | Non |  |
| search | chaîne | requête | Non |  |
| sso | chaîne | requête | Non |  |

## Response

Returns: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemple getPollVoters'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String |  (optionnel)
let page = 987 // Int |  (optionnel)
let search = "search_example" // String |  (optionnel)
let sso = "sso_example" // String |  (optionnel)

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