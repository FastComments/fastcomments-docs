## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Nee |  |
| byIPFromComment | string | query | Nee |  |
| filters | string | query | Nee |  |
| searchFilters | string | query | Nee |  |
| afterId | string | query | Nee |  |
| demo | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getApiIds Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De onderstaande codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (optioneel)
let byIPFromComment = "byIPFromComment_example" // String |  (optioneel)
let filters = "filters_example" // String |  (optioneel)
let searchFilters = "searchFilters_example" // String |  (optioneel)
let afterId = "afterId_example" // String |  (optioneel)
let demo = true // Bool |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

ModerationAPI.getApiIds(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, afterId: afterId, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]