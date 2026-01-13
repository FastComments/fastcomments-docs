Carica e ridimensiona un'immagine

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| sizePreset | string | query | No | Preset dimensione: "Default" (1000x1000px) oppure "CrossPlatform" (crea dimensioni per dispositivi popolari) |
| urlId | string | query | No | ID pagina da cui avviene l'upload, per la configurazione |

## Risposta

Restituisce: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalare tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Preset dimensione: \"Default\" (1000x1000px) oppure \"CrossPlatform\" (crea dimensioni per dispositivi popolari) (opzionale)
let urlId = "urlId_example" // String | ID pagina da cui avviene l'upload, per la configurazione (opzionale)

PublicAPI.uploadImage(tenantId: tenantId, file: file, sizePreset: sizePreset, urlId: urlId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]