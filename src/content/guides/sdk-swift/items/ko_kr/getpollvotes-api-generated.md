하나의 투표 집계 뒤에 있는 개별 투표 기록으로, 오래된 순서부터 정렬됩니다.

투표는 댓글에 속하므로, 투표 기록은 항상 한 번에 하나의 투표만 조회합니다 - commentId가 필수입니다. 이를 통해 모든 쿼리가 컬렉션이 이미 보유한 인덱스를 사용하게 됩니다.

투표의 프라이버시 설정을 따릅니다: 익명 투표의 기록은 여기서나 id로 조회할 수 없습니다 (poll-anonymous).

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 예 |  |
| voterId | string | query | 아니요 |  |
| optionId | string | query | 아니요 |  |
| skip | integer | query | 아니요 |  |

## 응답

반환값: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## 예제

[inline-code-attrs-start title = 'getPollVotes 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (optional)
let optionId = "optionId_example" // String |  (optional)
let skip = 987 // Int |  (optional)

DefaultAPI.getPollVotes(tenantId: tenantId, commentId: commentId, options: DefaultAPI.GetPollVotesOptions(voterId: voterId, optionId: optionId, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]