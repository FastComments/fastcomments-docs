---
## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | 아니오 |  |
| byIPFromComment | string | query | 아니오 |  |
| filters | string | query | 아니오 |  |
| searchFilters | string | query | 아니오 |  |
| afterId | string | query | 아니오 |  |
| demo | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## 예제

[inline-code-attrs-start title = 'getApiIds 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (선택 사항)
let byIPFromComment = "byIPFromComment_example" // String |  (선택 사항)
let filters = "filters_example" // String |  (선택 사항)
let searchFilters = "searchFilters_example" // String |  (선택 사항)
let afterId = "afterId_example" // String |  (선택 사항)
let demo = true // Bool |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

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

---