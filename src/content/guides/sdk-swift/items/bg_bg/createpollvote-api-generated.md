Запишете глас за анкета или преместете съществуващ глас към друга опция. Гласуващият може да има най‑много един глас за анкета, така че повторното извикване за същия гласуващ премества гласа му, вместо да добавя нов.

Това спазва настройките на анкетата в сайта: ако гласуването е разрешено само за вписани потребители, глас с само anonUserId се отказва, а анонимните гласове са ограничени по брой за IP за всяка анкета.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes |  |

## Отговор

Връща: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за createPollVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са в бета. При какъвто и да е проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
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