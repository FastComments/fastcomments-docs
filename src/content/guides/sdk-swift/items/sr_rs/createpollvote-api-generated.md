Запишите глас у анкети, или преместите постојећи глас на другу опцију. Гласач може имати највише један глас по анкети, тако да поновним позивом за истог гласача премешта се његов глас уместо додавања новог.

Ово поштује подешавања анкете на сајту: ако је гласање омогућено само пријављеним корисницима, глас са само anonUserId се одбија, а анонимни гласови су ограничени по IP по анкети.

## Parameters

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Да |  |

## Response

Враћа: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Пример

[inline-code-attrs-start title = 'Primer createPollVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su i dalje beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
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