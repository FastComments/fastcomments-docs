The individual votes behind one poll's tallies, oldest first.

하나의 설문 조사 집계 뒤에 있는 개별 투표들, 오래된 순서대로.

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

설문 조사는 댓글에 속하므로 투표는 항상 한 번에 하나의 설문 조사씩 읽습니다 - commentId가 필요합니다. 이렇게 하면 컬렉션이 이미 가지고 있는 인덱스에서 모든 쿼리를 수행할 수 있습니다.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

설문 조사 프라이버시를 준수합니다: 익명 설문 조사(poll-anonymous)의 투표는 여기서든 ID를 통해서든 읽을 수 없습니다.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| voterId | string | query | No |  |
| optionId | string | query | No |  |
| skip | integer | query | No |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPollVotes 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (옵션)
let optionId = "optionId_example" // String |  (옵션)
let skip = 987 // Int |  (옵션)

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