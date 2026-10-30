---
Bir ankette oy kaydedin veya mevcut bir oyu farklı bir seçeneğe taşıyın. Bir seçmen bir anket başına en fazla bir oy alabilir, bu yüzden aynı seçmen için bu yöntemi tekrar çağırmak, oy eklemek yerine mevcut oyunu taşır.

Bu, sitenin anket ayarlarına uyar: oy verme yalnızca oturum açmış kullanıcılar için ayarlanmışsa, sadece anonUserId içeren bir oy reddedilir ve anonim oylar IP başına anket başına sınırlanır.

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Yanıt

Döndürür: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Örnek

[inline-code-attrs-start title = 'createPollVote Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
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

---