## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| sso | string | query | Ne |  |

## Response

Vrača: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AdjustVotesResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer postAdjustCommentVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta. Za vsako težavo poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let adjustCommentVotesParams = AdjustCommentVotesParams(adjustVoteAmount: 123) // AdjustCommentVotesParams | 
let sso = "sso_example" // String |  (neobvezno)

ModerationAPI.postAdjustCommentVotes(commentId: commentId, adjustCommentVotesParams: adjustCommentVotesParams, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]