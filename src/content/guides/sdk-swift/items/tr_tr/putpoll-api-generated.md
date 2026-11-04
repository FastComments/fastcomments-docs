Bir ankete yorum ekleyin veya zaten sahip olduğu anketin tüm durumunu ayarlayın.

Seçenekler id ile eşleştirilir: mevcut bir seçeneğin id'si ile gönderilen seçenek oylarını korur (ve yeni etiket ile konumu alır), id olmadan gönderilen seçenek eklenir ve listede yer almayan mevcut seçenekler, üzerine verilen oylarla birlikte kaldırılır.

Oyu olan bir ankette hiçbir mevcut seçenek id'sinin tutulmaması tüm oyları siler, bu nedenle replaceVotes=true gereklidir.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | path | Evet |  |
| replaceVotes | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Örnek

[inline-code-attrs-start title = 'putPoll Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
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