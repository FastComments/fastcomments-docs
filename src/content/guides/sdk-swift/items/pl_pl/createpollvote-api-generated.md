Zagłosuj w ankiecie lub przenieś istniejący głos na inną opcję. Głosujący ma co najwyżej jeden głos na ankietę, więc ponowne wywołanie dla tego samego głosującego przenosi jego głos, a nie dodaje nowego.

Funkcja respektuje ustawienia ankiet witryny: jeśli głosowanie jest ograniczone do zalogowanych użytkowników, głos z samym anonUserId zostanie odrzucony, a anonimowe głosy są ograniczone limitem na IP na ankietę.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład createPollVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W przypadku problemów zgłoś je na http://github.com/OpenAPITools/openapi-generator/issues/new
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