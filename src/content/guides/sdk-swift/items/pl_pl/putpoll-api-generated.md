Attach a poll to a comment, or set the full state of the poll it already has.

Opcje są dopasowywane po identyfikatorze: opcja wysłana z identyfikatorem istniejącej opcji zachowuje swoje głosy (i przyjmuje nową etykietę oraz pozycję), opcja wysłana bez identyfikatora jest dodawana, a istniejące opcje pominięte w liście są usuwane wraz z oddanymi na nie głosami.

Brak istniejących identyfikatorów opcji w ankiecie, która ma głosy, usuwa wszystkie z nich, dlatego konieczne jest użycie replaceVotes=true.

## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| commentId | string | path | Tak |  |
| replaceVotes | boolean | query | Nie |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Example

[inline-code-attrs-start title = 'Przykład putPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W przypadku jakichkolwiek problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (opcjonalnie)

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