The individual votes behind one poll's tallies, oldest first.

Un sondage appartient à un commentaire, donc les votes sont toujours lus un sondage à la fois - commentId est requis. Cela maintient chaque requête sur les index que la collection possède déjà.

Respecte la confidentialité du sondage : les votes d'un sondage anonyme ne peuvent pas être lus (poll-anonymous), ici ou par identifiant.

## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | query | Oui |  |
| voterId | string | query | Non |  |
| optionId | string | query | Non |  |
| skip | integer | query | Non |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemple getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (optionnel)
let optionId = "optionId_example" // String |  (optionnel)
let skip = 987 // Int |  (optionnel)

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