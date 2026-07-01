Upload and resize an image
============================

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| sizePreset | string | query | Non | Préréglage de taille : \"Default\" (1000x1000px) ou \"CrossPlatform\" (crée des tailles pour les appareils populaires) |
| urlId | string | query | Non | ID de page à partir de laquelle le téléversement a lieu, pour configurer |

## Réponse

Retourne : [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Préréglage de taille : \"Default\" (1000x1000px) ou \"CrossPlatform\" (crée des tailles pour les appareils populaires) (optionnel)
let urlId = "urlId_example" // String | ID de page à partir de laquelle le téléversement a lieu, pour configurer (optionnel)

PublicAPI.uploadImage(tenantId: tenantId, file: file, options: PublicAPI.UploadImageOptions(sizePreset: sizePreset, urlId: urlId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]