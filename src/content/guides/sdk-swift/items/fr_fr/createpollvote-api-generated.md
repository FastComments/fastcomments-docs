---
Enregistrez un vote sur un sondage, ou déplacez un vote existant vers une option différente. Un électeur ne peut avoir qu'un seul vote par sondage, ainsi appeler cette fonction à nouveau pour le même électeur déplace son vote plutôt que d'en ajouter un.

Cela respecte les paramètres de sondage du site : si le vote est limité aux utilisateurs connectés, un vote avec uniquement un anonUserId est rejeté, et les votes anonymes sont limités en fréquence par IP pour chaque sondage.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |

## Réponse

Retourne : [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Exemple

[inline-code-attrs-start title = 'createPollVote Exemple'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
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