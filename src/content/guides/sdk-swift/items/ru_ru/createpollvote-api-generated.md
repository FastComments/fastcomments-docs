Записать голос в опросе или переместить существующий голос к другому варианту. У избирателя может быть не более одного голоса в каждом опросе, поэтому повторный вызов для того же избирателя перемещает его голос, а не добавляет новый.

Это соблюдает настройки опросов сайта: если голосование разрешено только для авторизованных пользователей, голос с только anonUserId отклоняется, а анонимные голоса ограничиваются по частоте запросов с одного IP для каждого опроса.

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Ответ

Возвращает: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Пример

[inline-code-attrs-start title = 'createPollVote Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода находятся в бета‑версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
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