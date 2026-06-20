## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| userId | string | query | 아니요 |  |
| direction | string | query | 아니요 |  |
| repliesToUserId | string | query | 아니요 |  |
| page | number | query | 아니요 |  |
| includei10n | boolean | query | 아니요 |  |
| locale | string | query | 아니요 |  |
| isCrawler | boolean | query | 아니요 |  |

## 응답

반환: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## 예제

[inline-code-attrs-start title = 'getCommentsForUser 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new에 보고해 주세요
import FastCommentsSwift

let userId = "userId_example" // String |  (선택 사항)
let direction = SortDirections() // SortDirections |  (선택 사항)
let repliesToUserId = "repliesToUserId_example" // String |  (선택 사항)
let page = 987 // Double |  (선택 사항)
let includei10n = true // Bool |  (선택 사항)
let locale = "locale_example" // String |  (선택 사항)
let isCrawler = true // Bool |  (선택 사항)

PublicAPI.getCommentsForUser(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]