req
tenantId
afterId

## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| afterId | string | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| tags | array | query | 아니오 |  |

## 응답

반환: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsResponse.swift)

## 예제

[inline-code-attrs-start title = 'getFeedPosts 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // 문자열 | 
let afterId = "afterId_example" // 문자열 |  (선택 사항)
let limit = 987 // 정수 |  (선택 사항)
let tags = ["inner_example"] // [문자열] |  (선택 사항)

DefaultAPI.getFeedPosts(tenantId: tenantId, afterId: afterId, limit: limit, tags: tags) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]