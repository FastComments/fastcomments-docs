## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| repliesToUserId | string | query | 아니오 |  |
| page | number | query | 아니오 |  |
| includei10n | boolean | query | 아니오 |  |
| locale | string | query | 아니오 |  |
| isCrawler | boolean | query | 아니오 |  |

## 응답

Returns: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## 예시

[inline-code-attrs-start title = 'getCommentsForUser 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해 주세요
import FastCommentsSwift

let userId = "userId_example" // String | (옵션)
let direction = SortDirections() // SortDirections | (옵션)
let repliesToUserId = "repliesToUserId_example" // String | (옵션)
let page = 987 // Double | (옵션)
let includei10n = true // Bool | (옵션)
let locale = "locale_example" // String | (옵션)
let isCrawler = true // Bool | (옵션)

PublicAPI.getCommentsForUser(options: PublicAPI.GetCommentsForUserOptions(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]