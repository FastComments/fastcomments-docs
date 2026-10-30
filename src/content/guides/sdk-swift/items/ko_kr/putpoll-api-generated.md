---
Comment에 설문을 첨부하거나 이미 존재하는 설문의 전체 상태를 설정합니다.

옵션은 id로 매칭됩니다: 기존 옵션의 id와 함께 전송된 옵션은 투표 수를 유지하고(새 라벨과 위치를 적용), id 없이 전송된 옵션은 추가되며, 목록에 포함되지 않은 기존 옵션은 해당 옵션에 대한 투표와 함께 삭제됩니다.

투표가 있는 설문에서 기존 옵션 id를 전혀 남기지 않으면 모든 옵션이 삭제되므로, 이 경우 `replaceVotes=true`가 필요합니다.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| replaceVotes | boolean | query | No |  |

## Response

반환: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Example

[inline-code-attrs-start title = 'putPoll 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (optional)

DefaultAPI.putPoll(tenantId: tenantId, commentId: commentId, commentPollPutInput: commentPollPutInput, replaceVotes: replaceVotes) { (response, error) in
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