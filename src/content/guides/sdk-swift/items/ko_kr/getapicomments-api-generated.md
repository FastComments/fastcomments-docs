## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| page | number | query | 아니요 |  |
| count | number | query | 아니요 |  |
| text-search | string | query | 아니요 |  |
| byIPFromComment | string | query | 아니요 |  |
| filters | string | query | 아니요 |  |
| searchFilters | string | query | 아니요 |  |
| sorts | string | query | 아니요 |  |
| demo | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## Response

Returns: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Example

[inline-code-attrs-start title = 'getApiComments 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Double | (선택 사항)
let count = 987 // Double | (선택 사항)
let textSearch = "textSearch_example" // String | (선택 사항)
let byIPFromComment = "byIPFromComment_example" // String | (선택 사항)
let filters = "filters_example" // String | (선택 사항)
let searchFilters = "searchFilters_example" // String | (선택 사항)
let sorts = "sorts_example" // String | (선택 사항)
let demo = true // Bool | (선택 사항)
let sso = "sso_example" // String | (선택 사항)

ModerationAPI.getApiComments(tenantId: tenantId, options: ModerationAPI.GetApiCommentsOptions(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]