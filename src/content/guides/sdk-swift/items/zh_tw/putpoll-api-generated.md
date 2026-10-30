---
將投票附加到評論，或設定其已存在投票的完整狀態。

選項會依據 id 進行匹配：若傳送的選項帶有既有選項的 id，則保留其投票（並採用新的標籤與位置）；若傳送的選項未帶 id，則會新增；而在列表中省略的既有選項則會被移除，連同其已投的票一起刪除。

在已有投票的投票中若不保留任何既有選項的 id，會刪除所有選項，因此需要將 replaceVotes 設為 true。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| replaceVotes | boolean | query | No |  |

## 回應

返回：[`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## 範例

[inline-code-attrs-start title = 'putPoll 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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