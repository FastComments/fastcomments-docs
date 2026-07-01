Carica e ridimensiona un'immagine

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| sizePreset | string | query | No | Preset di dimensione: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea dimensioni per dispositivi popolari) |
| urlId | string | query | No | ID pagina da cui avviene il caricamento, da configurare |

## Risposta

Restituisce: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalarlo via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Preset di dimensione: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea dimensioni per dispositivi popolari) (opzionale)
let urlId = "urlId_example" // String | ID pagina da cui avviene il caricamento, da configurare (opzionale)

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