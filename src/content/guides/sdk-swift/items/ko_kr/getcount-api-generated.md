## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filter | string | query | No |  |
| searchFilters | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## 응답

반환: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICountCommentsResponse.swift)

## 예제

[inline-code-attrs-start title = 'getCount 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 아래 코드 샘플은 아직 베타입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 신고해 주세요
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (선택 사항)
let byIPFromComment = "byIPFromComment_example" // String |  (선택 사항)
let filter = "filter_example" // String |  (선택 사항)
let searchFilters = "searchFilters_example" // String |  (선택 사항)
let demo = true // Bool |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

ModerationAPI.getCount(textSearch: textSearch, byIPFromComment: byIPFromComment, filter: filter, searchFilters: searchFilters, demo: demo, sso: sso) { (response, error) in
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