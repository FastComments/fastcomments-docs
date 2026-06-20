## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationCommentSearchResponse.swift)

## Example

[inline-code-attrs-start title = 'getSearchCommentsSummary Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (optional)
let filters = "filters_example" // String |  (optional)
let searchFilters = "searchFilters_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

ModerationAPI.getSearchCommentsSummary(value: value, filters: filters, searchFilters: searchFilters, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
