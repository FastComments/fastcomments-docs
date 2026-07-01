## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | Yes |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicAPISetCommentTextResponse.swift)

## Primjer

[inline-code-attrs-start title = 'setCommentText primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći kod primjeri su još u beta fazi. Za bilo koji problem, molimo prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let commentTextUpdateRequest = CommentTextUpdateRequest(comment: "comment_example", mentions: [CommentUserMentionInfo(id: "id_example", tag: "tag_example", rawTag: "rawTag_example", type: "type_example", sent: false)], hashTags: [CommentUserHashTagInfo(id: "id_example", tag: "tag_example", url: "url_example", retain: false)]) // CommentTextUpdateRequest | 
let editKey = "editKey_example" // String |  (opcionalno)
let sso = "sso_example" // String |  (opcionalno)

PublicAPI.setCommentText(tenantId: tenantId, commentId: commentId, broadcastId: broadcastId, commentTextUpdateRequest: commentTextUpdateRequest, options: PublicAPI.SetCommentTextOptions(editKey: editKey, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]