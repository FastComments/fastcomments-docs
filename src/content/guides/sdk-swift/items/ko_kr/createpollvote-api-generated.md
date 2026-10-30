Record a vote on a poll, or move an existing one to a different option. A voter has at most one vote per
poll, so calling this again for the same voter moves their vote rather than adding one.

이 설문조사에 투표를 기록하거나 기존 투표를 다른 옵션으로 이동합니다. 투표자는 설문당 최대 하나의 투표만 할 수 있으므로, 동일한 투표자에 대해 다시 호출하면 투표가 추가되는 것이 아니라 기존 투표가 이동됩니다.

This obeys the site's poll settings: if voting is set to logged-in users only, a vote with just an
anonUserId is rejected, and anonymous votes are rate limited per IP per poll.

이는 사이트의 설문 설정을 따릅니다: 투표가 로그인한 사용자만 가능하도록 설정된 경우, anonUserId만으로는 투표가 거부되며, 익명 투표는 설문당 IP당 제한됩니다.

## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## 예시

[inline-code-attrs-start title = 'createPollVote 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createPollVoteBody = CreatePollVoteBody(commentId: "commentId_example", optionId: "optionId_example", userId: "userId_example", anonUserId: "anonUserId_example", ip: "ip_example") // CreatePollVoteBody | 

DefaultAPI.createPollVote(tenantId: tenantId, createPollVoteBody: createPollVoteBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]