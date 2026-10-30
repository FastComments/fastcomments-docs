Attachez un sondage à un commentaire, ou définissez l'état complet du sondage qu'il possède déjà.

Les options sont associées par identifiant : une option envoyée avec l'identifiant d'une option existante conserve ses votes (et prend le nouveau libellé et la nouvelle position), une option envoyée sans identifiant est ajoutée, et les options existantes omises de la liste sont supprimées ainsi que les votes qui y ont été attribués.

Ne conserver aucun identifiant d'option existant sur un sondage qui possède des votes supprime toutes les options, il faut donc que replaceVotes=true.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |
| replaceVotes | boolean | query | Non |  |

## Réponse

Retourne : [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple putPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (optionnel)

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