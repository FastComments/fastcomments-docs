---
## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIChildCommentsResponse.swift)

## Örnek

[inline-code-attrs-start title = 'postCommentsByIds Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let commentsByIdsParams = CommentsByIdsParams(ids: ["ids_example"]) // CommentsByIdsParams | 
let sso = "sso_example" // String |  (isteğe bağlı)

ModerationAPI.postCommentsByIds(commentsByIdsParams: commentsByIdsParams, sso: sso) { (response, error) in
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