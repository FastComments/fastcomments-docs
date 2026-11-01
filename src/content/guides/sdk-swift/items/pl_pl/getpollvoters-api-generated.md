Kto zagłosował na którą opcję. Anonimowe ankiety nigdy nie odpowiadają, ankiety tylko dla administratorów odpowiadają wyłącznie administratorowi lub moderatorowi.

## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | path | Tak |  |
| commentId | string | path | Tak |  |
| urlId | string | query | Nie |  |
| page | integer | query | Nie |  |
| search | string | query | Nie |  |
| sso | string | query | Nie |  |

## Response

Zwraca: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getPollVoters'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku jakichkolwiek problemów proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String |  (opcjonalny)
let page = 987 // Int |  (opcjonalny)
let search = "search_example" // String |  (opcjonalny)
let sso = "sso_example" // String |  (opcjonalny)

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