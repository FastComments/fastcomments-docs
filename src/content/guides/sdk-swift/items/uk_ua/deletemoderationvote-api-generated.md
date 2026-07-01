## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | запит | Так |  |
| commentId | string | шлях | Так |  |
| voteId | string | шлях | Так |  |
| broadcastId | string | запит | Ні |  |
| sso | string | запит | Ні |  |

## Відповідь

Повертає: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteDeleteResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteModerationVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду все ще бета. У разі будь-якої проблеми, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voteId = "voteId_example" // String | 
let broadcastId = "broadcastId_example" // String |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

ModerationAPI.deleteModerationVote(tenantId: tenantId, commentId: commentId, voteId: voteId, options: ModerationAPI.DeleteModerationVoteOptions(broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]