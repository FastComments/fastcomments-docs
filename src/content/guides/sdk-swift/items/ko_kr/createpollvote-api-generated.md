투표(poll)에 새 투표를 기록하거나, 기존 투표를 다른 옵션으로 이동합니다. 투표자는 투표당 최대 하나의 표만 가질 수 있으므로, 동일한 투표자에 대해 이 API를 다시 호출하면 표가 추가되는 것이 아니라 기존 표가 이동됩니다.

이 API는 사이트의 투표 설정을 따릅니다. 로그인한 사용자만 투표할 수 있도록 설정된 경우 anonUserId만 포함한 투표는 거부되며, 익명 투표는 투표별로 IP당 속도 제한이 적용됩니다.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

반환값: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Example

[inline-code-attrs-start title = 'createPollVote 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
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