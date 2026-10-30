The individual votes behind one poll's tallies, oldest first.

Bir anket bir yoruma aittir, bu yüzden oylar her zaman bir anket başına okunur - commentId gereklidir. Bu, her sorgunun koleksiyonun zaten sahip olduğu indekslerde kalmasını sağlar.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | query | Evet |  |
| voterId | string | query | Hayır |  |
| optionId | string | query | Hayır |  |
| skip | integer | query | Hayır |  |

## Response

Döndürür: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getPollVotes Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta sürümündedir. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (opsiyonel)
let optionId = "optionId_example" // String |  (opsiyonel)
let skip = 987 // Int |  (opsiyonel)

DefaultAPI.getPollVotes(tenantId: tenantId, commentId: commentId, options: DefaultAPI.GetPollVotesOptions(voterId: voterId, optionId: optionId, skip: skip)) { (response, error) in
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