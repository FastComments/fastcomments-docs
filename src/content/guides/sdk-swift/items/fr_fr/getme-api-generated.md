---
Identifie les informations d’identification utilisées : le locataire auquel elles appartiennent et, pour les jetons OAuth, l’utilisateur qui les a autorisées.  
Les intégrations utilisent cela pour tester une connexion et l’étiqueter.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|------------|--------------|-------------|
| tenantId | string | query | Oui |  |

## Réponse

Renvoie : [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getMe'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
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