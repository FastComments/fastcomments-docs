## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## 응답

반환: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## 예제

[inline-code-attrs-start title = 'getApiComments 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let page = 987 // Double |  (선택 사항)
let count = 987 // Double |  (선택 사항)
let textSearch = "textSearch_example" // String |  (선택 사항)
let byIPFromComment = "byIPFromComment_example" // String |  (선택 사항)
let filters = "filters_example" // String |  (선택 사항)
let searchFilters = "searchFilters_example" // String |  (선택 사항)
let sorts = "sorts_example" // String |  (선택 사항)
let demo = true // Bool |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

ModerationAPI.getApiComments(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]