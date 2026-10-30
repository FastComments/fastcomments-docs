Cofnij głos. Opcja, na którą został oddany, zwraca swoją liczbę głosów.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |

## Odpowiedź

Zwraca: [`DeletePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeletePollVoteResponse.swift)

## Przykład

[inline-code-attrs-start title = 'deletePollVote Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku jakichkolwiek problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.deletePollVote(tenantId: tenantId, id: id) { (response, error) in
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