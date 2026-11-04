Stimme zu einer Umfrage ab oder verschiebe eine bestehende Stimme zu einer anderen Option. Ein Wähler hat maximal eine Stimme pro
Umfrage, daher verschiebt ein erneuter Aufruf für denselben Wähler seine Stimme, anstatt eine weitere hinzuzufügen.

Dies folgt den Umfrageeinstellungen der Website: Wenn die Abstimmung nur für angemeldete Benutzer erlaubt ist, wird eine Stimme nur mit
anonUserId abgelehnt, und anonyme Stimmen werden pro IP pro Umfrage ratenbegrenzt.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'createPollVote Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch im Beta-Stadium. Bei Problemen bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new melden
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