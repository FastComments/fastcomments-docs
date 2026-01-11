req
tenantId
urlId

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nee |  |
| direction | string | query | Nee |  |
| sso | string | query | Nee |  |
| skip | integer | query | Nee |  |
| skipChildren | integer | query | Nee |  |
| limit | integer | query | Nee |  |
| limitChildren | integer | query | Nee |  |
| countChildren | boolean | query | Nee |  |
| fetchPageForCommentId | string | query | Nee |  |
| includeConfig | boolean | query | Nee |  |
| countAll | boolean | query | Nee |  |
| includei10n | boolean | query | Nee |  |
| locale | string | query | Nee |  |
| modules | string | query | Nee |  |
| isCrawler | boolean | query | Nee |  |
| includeNotificationCount | boolean | query | Nee |  |
| asTree | boolean | query | Nee |  |
| maxTreeDepth | integer | query | Nee |  |
| useFullTranslationIds | boolean | query | Nee |  |
| parentId | string | query | Nee |  |
| searchText | string | query | Nee |  |
| hashTags | array | query | Nee |  |
| userId | string | query | Nee |  |
| customConfigStr | string | query | Nee |  |
| afterCommentId | string | query | Nee |  |
| beforeCommentId | string | query | Nee |  |

## Antwoord

Retourneert: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog beta. Voor problemen, meld deze via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (optioneel)
let direction = SortDirections() // SortDirections |  (optioneel)
let sso = "sso_example" // String |  (optioneel)
let skip = 987 // Int |  (optioneel)
let skipChildren = 987 // Int |  (optioneel)
let limit = 987 // Int |  (optioneel)
let limitChildren = 987 // Int |  (optioneel)
let countChildren = true // Bool |  (optioneel)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (optioneel)
let includeConfig = true // Bool |  (optioneel)
let countAll = true // Bool |  (optioneel)
let includei10n = true // Bool |  (optioneel)
let locale = "locale_example" // String |  (optioneel)
let modules = "modules_example" // String |  (optioneel)
let isCrawler = true // Bool |  (optioneel)
let includeNotificationCount = true // Bool |  (optioneel)
let asTree = true // Bool |  (optioneel)
let maxTreeDepth = 987 // Int |  (optioneel)
let useFullTranslationIds = true // Bool |  (optioneel)
let parentId = "parentId_example" // String |  (optioneel)
let searchText = "searchText_example" // String |  (optioneel)
let hashTags = ["inner_example"] // [String] |  (optioneel)
let userId = "userId_example" // String |  (optioneel)
let customConfigStr = "customConfigStr_example" // String |  (optioneel)
let afterCommentId = "afterCommentId_example" // String |  (optioneel)
let beforeCommentId = "beforeCommentId_example" // String |  (optioneel)

PublicAPI.getCommentsPublic(tenantId: tenantId, urlId: urlId, page: page, direction: direction, sso: sso, skip: skip, skipChildren: skipChildren, limit: limit, limitChildren: limitChildren, countChildren: countChildren, fetchPageForCommentId: fetchPageForCommentId, includeConfig: includeConfig, countAll: countAll, includei10n: includei10n, locale: locale, modules: modules, isCrawler: isCrawler, includeNotificationCount: includeNotificationCount, asTree: asTree, maxTreeDepth: maxTreeDepth, useFullTranslationIds: useFullTranslationIds, parentId: parentId, searchText: searchText, hashTags: hashTags, userId: userId, customConfigStr: customConfigStr, afterCommentId: afterCommentId, beforeCommentId: beforeCommentId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]